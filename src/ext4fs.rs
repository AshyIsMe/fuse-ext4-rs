// Ext4FS :: ext4 fuse filesystem
//
//

use std::convert::TryFrom;
use std::ffi::OsString;
use std::io;
use std::path::Path;

use anyhow::anyhow;
use anyhow::Context;
use ext4::Enhanced;
use ext4::FileType as ExtFileType;
use fuse_mt::*;
use positioned_io::ReadAt;

pub trait BlockDevice: ReadAt + Send + Sync {}
impl<T: ReadAt + Send + Sync> BlockDevice for T {}

impl ReadAt for Box<dyn BlockDevice> {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> io::Result<usize> {
        (**self).read_at(pos, buf)
    }
}

pub struct Ext4FS {
    pub target: OsString,
    pub superblock: ext4::SuperBlock<Box<dyn BlockDevice>>,
}

impl Ext4FS {
    pub fn new(target: OsString) -> anyhow::Result<Self> {
        let metadata = std::fs::metadata(&target)?;
        let mut block_device = std::fs::File::open(&target).unwrap();

        let partitions =
            match bootsector::list_partitions(&mut block_device, &bootsector::Options::default()) {
                Ok(parts) => parts,
                Err(e) if io::ErrorKind::NotFound == e.kind() => vec![],
                Err(e) => Err(e).with_context(|| anyhow!("searching for partitions"))?,
            };

        let block_device: Box<dyn BlockDevice> = if partitions.len() == 0 {
            Box::new(block_device)
        } else {
            let part = partitions
                .get(0)
                .ok_or_else(|| anyhow!("there wasn't at least one partition"))?;
            Box::new(positioned_io::Slice::new(
                block_device,
                part.first_byte,
                Some(part.len),
            ))
        };

        let superblock =
            ext4::SuperBlock::new(block_device).with_context(|| anyhow!("opening partition"))?;

        Ok(Self { target, superblock })
    }
}

const ONLY_FH: u64 = 5318008;

impl FilesystemMT for Ext4FS {
    fn init(&self, _req: RequestInfo) -> ResultEmpty {
        Ok(())
    }

    fn getattr(&self, _req: RequestInfo, path: &Path, _fh: Option<u64>) -> ResultEntry {
        println!("{:?}", path);
        if path != Path::new("/") {
            return Err(libc::ENOENT);
        }

        #AA TODO load_inode and get Stat

        let result = self.superblock.resolve_path(path.to_str().unwrap());
        match result {
            Ok(dirent) => {
                let i = dirent.inode.into();
                let inode = self.superblock.load_inode(i)?;
                Ok((
                    time::Timespec::new(0, 0),
                    FileAttr {
                        size: inode.stat.size,
                        blocks: 0,
                        atime: time::Timespec::new(0, 0),
                        mtime: time::Timespec::new(0, 0),
                        ctime: time::Timespec::new(0, 0),
                        crtime: time::Timespec::new(0, 0),
                        kind: FileType::Directory,
                        perm: 0o0777,
                        nlink: 0,
                        uid: 666,
                        gid: 666,
                        rdev: 0,
                        flags: 0,
                    },
                ))
            }
            Err(e) => {
                println!("getattr() error: {:?}", e);
                Err(libc::ENOENT)
            }
        }
    }

    fn opendir(&self, _req: RequestInfo, path: &Path, _flags: u32) -> ResultOpen {
        // let inode = self.superblock.resolve_path(path.to_str().unwrap()).unwrap().inode;

        // match self.superblock.resolve_path(path.to_str().unwrap()) {
        let result = self.superblock.resolve_path(path.to_str().unwrap());
        match result {
            Ok(dirent) => {
                // TODO inode is not a filehandle (maybe?) (absolutely not)
                let filehandle = dirent.inode.into();
                Ok((filehandle, 0))
            }
            Err(e) => {
                println!("opendir() error: {:?}", e);
                Err(libc::ENOENT)
            }
        }
    }

    fn readdir(&self, _req: RequestInfo, _path: &Path, fh: u64) -> ResultReaddir {
        let inode = self
            .superblock
            .load_inode(u32::try_from(fh).expect("definitely not an inode"))
            .expect("invalid inode");
        match self.superblock.enhance(&inode).expect("valid inode") {
            Enhanced::Directory(entries) => Ok(entries
                .into_iter()
                .map(|e| DirectoryEntry {
                    name: OsString::from(e.name),
                    kind: match e.file_type {
                        ExtFileType::RegularFile => FileType::RegularFile,
                        ExtFileType::SymbolicLink => FileType::Symlink,
                        ExtFileType::CharacterDevice => FileType::CharDevice,
                        ExtFileType::BlockDevice => FileType::BlockDevice,
                        ExtFileType::Directory => FileType::Directory,
                        // TODO: maybe?
                        ExtFileType::Fifo => FileType::NamedPipe,
                        ExtFileType::Socket => FileType::Socket,
                    },
                })
                .collect()),
            item => {
                eprintln!("readdir on non-directory {:?}", item);
                Err(libc::EBADE)
            }
        }
    }

    fn releasedir(&self, _req: RequestInfo, _path: &Path, fh: u64, _flags: u32) -> ResultEmpty {
        if ONLY_FH == fh {
            Ok(())
        } else {
            Err(libc::EBADF)
        }
    }
}

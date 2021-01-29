// Ext4FS :: ext4 fuse filesystem
//
//

use std::convert::TryFrom;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::io;
use std::path::Path;

use anyhow::anyhow;
use anyhow::Context;
use ext4::Enhanced;
use ext4::FileType as ExtFileType;
use fuse_mt::*;
use positioned_io::ReadAt;
use time::Timespec;

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

    fn destroy(&self, _req: RequestInfo) {}

    fn getattr(&self, _req: RequestInfo, path: &Path, _fh: Option<u64>) -> ResultEntry {
        println!("{:?}", path);
        if path != Path::new("/") {
            return Err(libc::ENOENT);
        }
        Ok((
            time::Timespec::new(0, 0),
            FileAttr {
                size: 0,
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

    fn chmod(&self, _req: RequestInfo, _path: &Path, _fh: Option<u64>, _mode: u32) -> ResultEmpty {
        Err(libc::EROFS)
    }

    fn chown(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: Option<u64>,
        _uid: Option<u32>,
        _gid: Option<u32>,
    ) -> ResultEmpty {
        Err(libc::EROFS)
    }

    fn truncate(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: Option<u64>,
        _size: u64,
    ) -> ResultEmpty {
        Err(libc::EROFS)
    }

    fn utimens(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: Option<u64>,
        _atime: Option<Timespec>,
        _mtime: Option<Timespec>,
    ) -> ResultEmpty {
        Err(libc::ENOSYS)
    }

    fn utimens_macos(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: Option<u64>,
        _crtime: Option<Timespec>,
        _chgtime: Option<Timespec>,
        _bkuptime: Option<Timespec>,
        _flags: Option<u32>,
    ) -> ResultEmpty {
        Err(libc::ENOSYS)
    }

    fn readlink(&self, _req: RequestInfo, _path: &Path) -> ResultData {
        Err(libc::ENOSYS)
    }

    fn mknod(
        &self,
        _req: RequestInfo,
        _parent: &Path,
        _name: &OsStr,
        _mode: u32,
        _rdev: u32,
    ) -> ResultEntry {
        Err(libc::EROFS)
    }

    fn mkdir(&self, _req: RequestInfo, _parent: &Path, _name: &OsStr, _mode: u32) -> ResultEntry {
        Err(libc::EROFS)
    }

    fn unlink(&self, _req: RequestInfo, _parent: &Path, _name: &OsStr) -> ResultEmpty {
        Err(libc::EROFS)
    }

    fn rmdir(&self, _req: RequestInfo, _parent: &Path, _name: &OsStr) -> ResultEmpty {
        Err(libc::EROFS)
    }

    fn symlink(
        &self,
        _req: RequestInfo,
        _parent: &Path,
        _name: &OsStr,
        _target: &Path,
    ) -> ResultEntry {
        Err(libc::EROFS)
    }

    fn rename(
        &self,
        _req: RequestInfo,
        _parent: &Path,
        _name: &OsStr,
        _newparent: &Path,
        _newname: &OsStr,
    ) -> ResultEmpty {
        Err(libc::EROFS)
    }

    fn link(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _newparent: &Path,
        _newname: &OsStr,
    ) -> ResultEntry {
        Err(libc::EROFS)
    }

    fn open(&self, _req: RequestInfo, _path: &Path, _flags: u32) -> ResultOpen {
        Err(libc::ENOSYS)
    }

    fn read(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: u64,
        _offset: u64,
        _size: u32,
        callback: impl FnOnce(ResultSlice<'_>) -> CallbackResult,
    ) -> CallbackResult {
        callback(Err(libc::ENOSYS))
    }

    fn write(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: u64,
        _offset: u64,
        _data: Vec<u8>,
        _flags: u32,
    ) -> ResultWrite {
        Err(libc::EROFS)
    }

    fn flush(&self, _req: RequestInfo, _path: &Path, _fh: u64, _lock_owner: u64) -> ResultEmpty {
        Err(libc::EROFS)
    }

    fn release(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _fh: u64,
        _flags: u32,
        _lock_owner: u64,
        _flush: bool,
    ) -> ResultEmpty {
        Err(libc::ENOSYS)
    }

    fn fsync(&self, _req: RequestInfo, _path: &Path, _fh: u64, _datasync: bool) -> ResultEmpty {
        Err(libc::EROFS)
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

    fn fsyncdir(&self, _req: RequestInfo, _path: &Path, _fh: u64, _datasync: bool) -> ResultEmpty {
        Err(libc::EROFS)
    }

    fn statfs(&self, _req: RequestInfo, _path: &Path) -> ResultStatfs {
        Err(libc::ENOSYS)
    }

    fn setxattr(
        &self,
        _req: RequestInfo,
        _path: &Path,
        _name: &OsStr,
        _value: &[u8],
        _flags: u32,
        _position: u32,
    ) -> ResultEmpty {
        Err(libc::EROFS)
    }

    fn getxattr(&self, _req: RequestInfo, _path: &Path, _name: &OsStr, _size: u32) -> ResultXattr {
        Err(libc::ENOSYS)
    }

    fn listxattr(&self, _req: RequestInfo, _path: &Path, _size: u32) -> ResultXattr {
        Err(libc::ENOSYS)
    }

    fn removexattr(&self, _req: RequestInfo, _path: &Path, _name: &OsStr) -> ResultEmpty {
        Err(libc::EROFS)
    }

    fn access(&self, _req: RequestInfo, _path: &Path, _mask: u32) -> ResultEmpty {
        Err(libc::ENOSYS)
    }

    fn create(
        &self,
        _req: RequestInfo,
        _parent: &Path,
        _name: &OsStr,
        _mode: u32,
        _flags: u32,
    ) -> ResultCreate {
        Err(libc::EROFS)
    }
}

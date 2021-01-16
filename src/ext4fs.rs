// Ext4FS :: ext4 fuse filesystem
//
//

use std::ffi::OsString;
use std::io::{self, Read, Seek};
use std::path::Path;

use anyhow::anyhow;
use anyhow::Context;
use fuse_mt::*;

pub trait BlockDevice: Read + Seek + Send + Sync {}
impl<T: Seek + Read + Send + Sync> BlockDevice for T {}

pub struct Ext4FS {
    pub target: OsString,
    pub superblock: ext4::SuperBlock<Box<dyn BlockDevice>>,
}

impl Ext4FS {
    pub fn new(target: OsString) -> anyhow::Result<Self> {
        let metadata = std::fs::metadata(&target)?;
        let mut block_device = std::io::BufReader::new(std::fs::File::open(&target).unwrap());

        let partitions =
            match bootsector::list_partitions(&mut block_device, &bootsector::Options::default()) {
                Ok(parts) => parts,
                Err(e) if io::ErrorKind::NotFound == e.kind() => vec![],
                Err(e) => Err(e).with_context(|| anyhow!("searching for partitions"))?,
            };

        let block_device: Box<dyn BlockDevice> = if partitions.len() == 0 {
            Box::new(block_device)
        } else {
            Box::new(
                bootsector::open_partition(
                    block_device,
                    partitions
                        .get(0)
                        .ok_or_else(|| anyhow!("there wasn't at least one partition"))?,
                )
                .map_err(|e| anyhow!("opening partition 0: {:?}", e))?,
            )
        };

        let superblock =
            ext4::SuperBlock::new(block_device).with_context(|| anyhow!("opening partition"))?;

        Ok(Self { target, superblock })
    }
}

impl FilesystemMT for Ext4FS {
    fn init(&self, _req: RequestInfo) -> ResultEmpty {
        Ok(())
    }

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
                perm: 0,
                nlink: 0,
                uid: 0,
                gid: 0,
                rdev: 0,
                flags: 0,
            },
        ))
    }
}

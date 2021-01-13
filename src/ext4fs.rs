// Ext4FS :: ext4 fuse filesystem
//
//

use std::ffi::OsString;
use std::io::{Read, Seek};
use std::path::Path;

use anyhow::anyhow;
use anyhow::Context;
use fuse_mt::*;

trait BlockDevice: Read + Seek {}

pub struct Ext4FS {
    pub target: OsString,
    pub superblock: ext4::SuperBlock<bootsector::RangeReader<std::io::BufReader<std::fs::File>>>,
}

impl Ext4FS {
    pub fn new(target: OsString) -> anyhow::Result<Self> {
        let metadata = std::fs::metadata(&target)?;
        let mut block_device = std::io::BufReader::new(std::fs::File::open(&target).unwrap());

        // TODO - this errors out instead of resulting in an empty Vec<>
        let partitions =
            bootsector::list_partitions(&mut block_device, &bootsector::Options::default())
                .with_context(|| anyhow!("searching for partitions"))?;

        let block_device = if partitions.len() == 0 {
            let partition = bootsector::Partition{
                id: 0,
                first_byte: 0,
                len: metadata.len(),
                attributes: bootsector::Attributes::MBR{ bootable: false, type_code: 0, },
            };
            bootsector::open_partition(
                block_device,
                &partition,
            )
            .map_err(|e| anyhow!("opening partition 0: {:?}", e))?
        } else {
            bootsector::open_partition(
                block_device,
                partitions
                    .get(0)
                    .ok_or_else(|| anyhow!("there wasn't at least one partition"))?,
            )
            .map_err(|e| anyhow!("opening partition 0: {:?}", e))?
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

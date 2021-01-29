// Ext4FS :: ext4 fuse filesystem
//
//

use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;

use anyhow::anyhow;
use anyhow::Context;
use ext4::Enhanced;
use ext4::SuperBlock;
use fuse_mt::*;
use time::Timespec;

use crate::block_device::open_raw_or_first_partition;
use crate::block_device::BlockDevice;
use crate::fh::inode_from_fh_or_path;
use crate::mappe::{map_kind, timespec};

pub struct Ext4FS {
    pub target: OsString,
    pub superblock: ext4::SuperBlock<Box<dyn BlockDevice>>,
}

impl Ext4FS {
    pub fn new(target: OsString) -> anyhow::Result<Self> {
        Ok(Self {
            superblock: SuperBlock::new(
                open_raw_or_first_partition(&target)
                    .with_context(|| anyhow!("opening file/partition"))?,
            )
            .with_context(|| anyhow!("loading filesystem"))?,
            target,
        })
    }
}

const ONLY_FH: u64 = 5318008;

impl FilesystemMT for Ext4FS {
    fn init(&self, _req: RequestInfo) -> ResultEmpty {
        Ok(())
    }

    fn destroy(&self, _req: RequestInfo) {}

    fn getattr(&self, _req: RequestInfo, path: &Path, fh: Option<u64>) -> ResultEntry {
        let stat = inode_from_fh_or_path(&self.superblock, fh, path)?.stat;
        Ok((
            time::Timespec::new(0, 0),
            FileAttr {
                size: stat.size,
                blocks: 0,
                atime: timespec(stat.atime),
                mtime: timespec(stat.mtime),
                ctime: timespec(stat.ctime),
                crtime: stat
                    .btime
                    .map(|t| timespec(t))
                    .unwrap_or(time::Timespec::new(0, 0)),
                kind: map_kind(stat.extracted_type),
                perm: stat.file_mode,
                nlink: u32::from(stat.link_count),
                uid: stat.uid,
                gid: stat.gid,
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

    fn readdir(&self, _req: RequestInfo, path: &Path, fh: u64) -> ResultReaddir {
        let inode = inode_from_fh_or_path(&self.superblock, Some(fh), path)?;
        match self.superblock.enhance(&inode).expect("valid inode") {
            Enhanced::Directory(entries) => Ok(entries
                .into_iter()
                .map(|e| DirectoryEntry {
                    name: OsString::from(e.name),
                    kind: map_kind(e.file_type),
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

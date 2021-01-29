use std::convert::TryFrom;

use ext4::FileType as ExtFileType;
use ext4::Time;
use fuse_mt::FileType;
use time::Timespec;

pub fn timespec(time: Time) -> Timespec {
    Timespec::new(
        i64::from(time.epoch_secs),
        time.nanos.and_then(|v| i32::try_from(v).ok()).unwrap_or(0),
    )
}

pub fn map_kind(file_type: ExtFileType) -> FileType {
    match file_type {
        ExtFileType::RegularFile => FileType::RegularFile,
        ExtFileType::SymbolicLink => FileType::Symlink,
        ExtFileType::CharacterDevice => FileType::CharDevice,
        ExtFileType::BlockDevice => FileType::BlockDevice,
        ExtFileType::Directory => FileType::Directory,
        // TODO: maybe?
        ExtFileType::Fifo => FileType::NamedPipe,
        ExtFileType::Socket => FileType::Socket,
    }
}

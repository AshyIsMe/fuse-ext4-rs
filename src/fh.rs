//! We're actually treating inodes as filehandles, because fuse doesn't expose
//! any filehandle stuff to us?

use std::convert::TryFrom;
use std::path::Path;

use ext4::SuperBlock;
use ext4::{Inode, ParseError};
use positioned_io::ReadAt;

fn inode_load<T: ReadAt>(fs: &SuperBlock<T>, inode: u32) -> Result<Inode, libc::c_int> {
    match fs.load_inode(inode) {
        Ok(inode) => Ok(inode),
        Err(e) => {
            eprintln!("unexpected error loading: {:?}", e);
            Err(libc::EIO)
        }
    }
}

pub fn inode_from_fh_or_path<T: ReadAt>(
    fs: &SuperBlock<T>,
    fh: Option<u64>,
    path: &Path,
) -> Result<Inode, libc::c_int> {
    match fh {
        Some(fh) => match u32::try_from(fh) {
            Ok(fh) => inode_load(fs, fh),
            Err(_) => {
                eprintln!("bad fh {:?}", fh);
                Err(libc::EBADF)
            }
        },
        None => match path.to_str() {
            None => {
                eprintln!("invalid utf-8 in path: {:?}", path);
                Err(libc::EINVAL)
            }
            Some(path) => match fs.resolve_path(path) {
                Err(e) => match e.downcast::<ParseError>() {
                    Ok(ParseError::NotFound { .. }) => Err(libc::ENOENT),
                    Ok(e) => {
                        eprintln!("unexpected error parsing: {:?}", e);
                        Err(libc::ENOSYS)
                    }
                    Err(e) => {
                        eprintln!("unexpected error resolving: {:?}", e);
                        Err(libc::EIO)
                    }
                },
                Ok(ent) => inode_load(fs, ent.inode),
            },
        },
    }
}

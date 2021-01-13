use std::ffi::OsStr;

use anyhow::Result;

mod ext4fs;

pub fn mount_and_run(what: &OsStr, whence: &OsStr) -> Result<()> {
    let filesystem = ext4fs::Ext4FS::new(what.to_os_string())?;
    let fuse_args: Vec<&OsStr> = vec![&OsStr::new("-o"), &OsStr::new("auto_unmount")];
    fuse_mt::mount(fuse_mt::FuseMT::new(filesystem, 1), &whence, &fuse_args)?;
    Ok(())
}

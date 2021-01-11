use std::env;
use std::ffi::{OsStr, OsString};

use anyhow::Result;

mod ext4fs;

fn start() -> Result<i32> {
    let args: Vec<OsString> = env::args_os().collect();

    if args.len() != 3 {
        println!(
            "usage: {} <target> <mountpoint>",
            &env::args().next().unwrap()
        );
        return Ok(2);
    }

    let filesystem = ext4fs::Ext4FS::new(args[1].clone());

    let fuse_args: Vec<&OsStr> = vec![&OsStr::new("-o"), &OsStr::new("auto_unmount")];

    fuse_mt::mount(fuse_mt::FuseMT::new(filesystem, 1), &args[2], &fuse_args)?;

    Ok(0)
}

fn main() -> Result<()> {
    ::std::process::exit(start()?)
}

use std::env;

use anyhow::Result;

fn start() -> Result<i32> {
    let args: Vec<_> = env::args_os().collect();

    if args.len() != 3 {
        println!(
            "usage: {} <target> <mountpoint>",
            &env::args().next().unwrap()
        );
        return Ok(2);
    }

    fuse_ext4_rs::mount_and_run(&args[1], &args[2])?;

    Ok(0)
}

fn main() -> Result<()> {
    ::std::process::exit(start()?)
}

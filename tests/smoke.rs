use std::fs;
use std::path::Path;
use std::thread;

use anyhow::Result;
use std::time::Duration;

#[test]
fn smoke() -> Result<()> {
    let dest = "mnt";
    std::process::Command::new("pkill")
        .args(&["-f", "gvfs"])
        .status()?;
    fs::create_dir_all(dest)?;
    let thread = thread::spawn(move || {
        fuse_ext4_rs::mount_and_run("tests/all-types-tiny.img".as_ref(), dest.as_ref())
    });
    thread::sleep(Duration::from_millis(500));
    let unmount_result = nix::mount::umount(dest);
    println!("{:?}", unmount_result);
    thread.join().unwrap()?;
    unmount_result?;

    Ok(())
}

use std::ffi::CString;
use std::fs;
use std::thread;
use std::time::Duration;

use anyhow::Result;

#[test]
fn smoke() -> Result<()> {
    let dest = "mnt";
    fs::create_dir_all(dest)?;
    let thread = thread::spawn(move || {
        fuse_ext4_rs::mount_and_run("tests/all-types-tiny.img".as_ref(), dest.as_ref())
    });
    thread::sleep(Duration::from_millis(500));
    unsafe {
        let cstr = CString::new(dest).expect("static string");
        cntr_fuse_sys::fuse_unmount_compat22(cstr.as_ptr())
    }
    thread.join().unwrap()?;

    Ok(())
}

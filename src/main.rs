use std::env;
use std::ffi::{OsStr, OsString};

use anyhow::Result;

mod ext4fs;

//fn main() {
//let r = fs::File::open(env::args().nth(1).expect("one argument")).expect("openable file");
//let mut options = ext4::Options::default();
//options.checksums = ext4::Checksums::Enabled;
//let mut vol = ext4::SuperBlock::new_with_options(r, &options).expect("ext4 volume");
//let root = vol.root().expect("root");
//vol.walk(&root, "/", &mut |_, path, _, _| {
//println!("{}", path);
//Ok(true)
//})
//.expect("walk");
//}

fn start() -> Result<i32> {
    let args: Vec<OsString> = env::args_os().collect();

    if args.len() != 3 {
        println!(
            "usage: {} <target> <mountpoint>",
            &env::args().next().unwrap()
        );
        return Ok(2);
    }

    let filesystem = ext4fs::Ext4FS {
        target: args[1].clone(),
    };

    let fuse_args: Vec<&OsStr> = vec![&OsStr::new("-o"), &OsStr::new("auto_unmount")];

    fuse_mt::mount(fuse_mt::FuseMT::new(filesystem, 1), &args[2], &fuse_args)?;

    Ok(0)
}

fn main() -> Result<()> {
    ::std::process::exit(start()?)
}

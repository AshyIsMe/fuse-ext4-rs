// Ext4FS :: ext4 fuse filesystem
//
//

use std::fs;
use std::ffi::{OsString};

use fuse_mt::*;

pub struct Ext4FS {
    pub target: OsString,
}

//impl PassthroughFS {
//}


impl FilesystemMT for Ext4FS {

    //fn init(&self, _req: RequestInfo) -> ResultEmpty {
        //println!("init");

        //let r = fs::File::open(&self.target).expect("openable file");
        //let mut options = ext4::Options::default();
        //options.checksums = ext4::Checksums::Enabled;
        //let mut vol = ext4::SuperBlock::new_with_options(r, &options).expect("ext4 volume");
        //let root = vol.root().expect("root");
        //vol.walk(&root, "/", &mut |_, path, _, _| {
            //println!("{}", path);
            //Ok(true)
        //})
        //.expect("walk");

        //Ok(())
    //}

    fn init(&self, _req: RequestInfo) -> ResultEmpty {
        println!("init");

        let mut block_device = std::io::BufReader::new(std::fs::File::open(&self.target).unwrap());
        let mut superblock = ext4::SuperBlock::new(&mut block_device).unwrap();
        let target_inode_number = superblock.resolve_path("/").unwrap().inode;
        println!("{}", target_inode_number);


        Ok(())
    }
}

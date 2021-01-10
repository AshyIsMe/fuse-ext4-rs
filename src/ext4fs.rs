// Ext4FS :: ext4 fuse filesystem
//
//

use std::ffi::OsString;
use std::fs;

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

    // let mut block_device = std::io::BufReader::new(std::fs::File::open(&self.target).unwrap());
    // let partitions = bootsector::list_partitions(&mut block_device, &bootsector::Options::default())
    // .with_context(|| anyhow!("searching for partitions"))?;
    // let block_device = bootsector::open_partition(block_device, partitions.get(0)
    // .ok_or_else(|| anyhow!("there wasn't at least one partition"))?)
    // .map_err(|e| anyhow!("opening partition 0: {:?}", e))?;

    fn init(&self, _req: RequestInfo) -> ResultEmpty {
        println!("init");

        let mut block_device = std::io::BufReader::new(std::fs::File::open(&self.target).unwrap());
        let partitions =
            bootsector::list_partitions(&mut block_device, &bootsector::Options::default())
                .expect("partitions");
        let mut block_device = bootsector::open_partition(
            block_device,
            partitions
                .get(0)
                .expect("there wasn't at least one partition"),
        )
        .expect("open partition");
        let mut superblock = ext4::SuperBlock::new(&mut block_device).unwrap();
        let target_inode_number = superblock.resolve_path("/").unwrap().inode;
        println!("{}", target_inode_number);

        Ok(())
    }
}

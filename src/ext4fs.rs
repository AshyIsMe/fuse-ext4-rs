// Ext4FS :: ext4 fuse filesystem
//
//

use std::ffi::OsString;
use std::fs::{File};

use fuse_mt::*;

pub struct Ext4FS {
    pub target: OsString,
    pub superblock: Option<ext4::SuperBlock<std::io::BufReader<std::fs::File>>>,
}

impl Ext4FS
{
    pub fn new(target: OsString) -> Self {
        println!("Ext4FS.new()");

        let block_device = std::io::BufReader::new(File::open(&target).unwrap());
        let superblock = ext4::SuperBlock::new(block_device).unwrap();

        Self {
            target,
            superblock: Some(superblock),
        }
    }
}

impl FilesystemMT for Ext4FS {

    fn init(&self, _req: RequestInfo) -> ResultEmpty {
        println!("init");

        Ok(())
    }
}

    // let mut block_device = std::io::BufReader::new(std::fs::File::open(&self.target).unwrap());
    // let partitions = bootsector::list_partitions(&mut block_device, &bootsector::Options::default())
    // .with_context(|| anyhow!("searching for partitions"))?;
    // let block_device = bootsector::open_partition(block_device, partitions.get(0)
    // .ok_or_else(|| anyhow!("there wasn't at least one partition"))?)
    // .map_err(|e| anyhow!("opening partition 0: {:?}", e))?;

    //fn init(&self, _req: RequestInfo) -> ResultEmpty {
        //println!("init");

        //let mut block_device = std::io::BufReader::new(std::fs::File::open(&self.target).unwrap());
        //let partitions =
            //bootsector::list_partitions(&mut block_device, &bootsector::Options::default())
                //.expect("partitions");
        //let mut block_device = bootsector::open_partition(
            //block_device,
            //partitions
                //.get(0)
                //.expect("there wasn't at least one partition"),
        //)
        //.expect("open partition");
        //let mut superblock = ext4::SuperBlock::new(&mut block_device).unwrap();
        //let target_inode_number = superblock.resolve_path("/").unwrap().inode;
        //println!("{}", target_inode_number);

        //Ok(())
    //}

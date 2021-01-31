use std::ffi::OsStr;
use std::io;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use positioned_io::ReadAt;

pub trait BlockDevice: ReadAt + Send + Sync {}
impl<T: ReadAt + Send + Sync> BlockDevice for T {}

pub fn open_raw_or_first_partition<S: AsRef<OsStr>>(target: S) -> Result<Box<dyn BlockDevice>> {
    let mut block_device = std::fs::File::open(target.as_ref()).unwrap();

    let partitions =
        match bootsector::list_partitions(&mut block_device, &bootsector::Options::default()) {
            Ok(parts) => parts,
            Err(e) if io::ErrorKind::NotFound == e.kind() => vec![],
            Err(e) => Err(e).with_context(|| anyhow!("searching for partitions"))?,
        };

    let block_device: Box<dyn BlockDevice> = if partitions.len() == 0 {
        Box::new(block_device)
    } else {
        let part = partitions
            .get(0)
            .ok_or_else(|| anyhow!("there wasn't at least one partition"))?;
        Box::new(positioned_io::Slice::new(
            block_device,
            part.first_byte,
            Some(part.len),
        ))
    };

    Ok(block_device)
}

impl ReadAt for Box<dyn BlockDevice> {
    fn read_at(&self, pos: u64, buf: &mut [u8]) -> io::Result<usize> {
        (**self).read_at(pos, buf)
    }
}

use aster_block::{id::BlockId, BlockDevice};
use aster_frame::vm::VmIo;
use bitvec::prelude::*;

const SFS_BLOCK_SIZE = 4096;
const SFS_MAX_INFO_LEN = 256;
const SFS_NDIRECT = 1024;
const SFS_MAX_FNAME_LEN = 256;

pub struct SFSSuperBlock {
    magic: u32,
    blocks: u32,
    unused_blocks: u32,
    info: [char; SFS_MAX_INFO_LEN+1],
}

pub struct SFSDiskInode {
    size: u32,
    file_type: u16,
    nlinks: u16,
    blocks: u32,
    direct: [u32; SFS_NDIRECT],
    indirect: u32,
}

pub struct SFSFileEntry {
    ino: u32,
    name: [char; SFS_MAX_FNAME_LEN+1];
}

pub type SFSFreemap = BitArray<[usize; SFS_BLOCK_SIZE]>;

pub struct SFS {
    block_device: Arc<dyn BlockDevice>,
    super_block: SFSSuperBlock,
    root_inode: SFSDiskInode,
    freemap: SFSFreemap,
}

impl SFS {
    pub fn open(block_device: &dyn BlockDevice) -> Result<Self> {
        let mut super_block = Self::read_super_block(block_device)?;
        if super_block.magic != 0xdeadbeef {
            Self::format(block_device)?;
            super_block = Self::read_super_block(block_device)?;
        }
        Ok(Self {
            block_device,
            super_block,
            root_inode: Self::read_root_inode(block_device)?,
            freemap: Self::read_freemap(block_device)?,
        })
    }
    pub fn format(block_device: &dyn BlockDevice) -> Result<()> {
        unimplemented!()
    }
    fn read_super_block(block_device: &dyn BlockDevice) -> Result<SFSSuperBlock> {
        block_device.read_val::<SFSSuperBlock>(0)
    }
    fn write_super_block(block_device: &dyn BlockDevice, super_block: &SFSSuperBlock) -> Result<()> {
        block_device.write_val(0, super_block)
    }
    fn read_root_inode(block_device: &dyn BlockDevice) -> Result<SFSDiskInode> {
        block_device.read_val::<SFSDiskInode>(1024)
    }
    fn write_inode(&self, ino: u32, inode: &SFSDiskInode) -> Result<()> {
        self.block_device.write_val(ino * SFS_BLOCK_SIZE, inode)
    }
}

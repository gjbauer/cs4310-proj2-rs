
const ins: usize = 2 * 4096;	// get_inode_start();

pub struct Inode {
	pub refs: u32,
	pub mode: u32,
	pub size: [u16; 2],
	pub ptrs: [u16; 2],
	pub iptr: u32,
	pub inum: u32,
}

pub fn inode_deserialize(mmap: &memmap2::MmapMut, num: usize) -> Inode {
	// TODO: Implement inode_deserialize...
}

pub fn inode_read(d: Inode, mmap: &memmap2::MmapMut) -> (Vec<u8>, u32) {
	// TODO: Read from an inode...
}




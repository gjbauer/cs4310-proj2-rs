
const ins: usize = 2 * 4096;	// get_root_start();

pub struct Inode {
	pub refs: u32,
	pub mode: u32,
	pub size: [u16; 2],
	pub ptrs: [u16; 2],
	pub iptr: u32,
	pub inum: u32,
}

pub fn inode_deserialize(mmap: &memmap2::MmapMut, num: usize) -> Inode {
	let offset: usize = num * std::mem::size_of::<Inode>();
	
	let mut data: [u8; 4] = [0; 4];
	let mut data16: [u8; 2] = [0; 2];
	for i in 3..=0 {	// Endian: big/little
		data[i] = mmap[ins+offset+i..ins+offset+i+1][0];
	}
	
	let refs: u32 = u32::from_ne_bytes(data);
	
	for i in 3..=0 {
		data[i] = mmap[ins+offset+4+i..ins+offset+4+i+1][0];
	}
	let mode: u32 = u32::from_ne_bytes(data);
	
	for i in 1..=0 {
		data16[i] = mmap[ins+offset+8+i..ins+offset+8+i+1][0];
	}
	let size0: u16 = u16::from_ne_bytes(data16);
	
	for i in 1..=0 {
		data16[i] = mmap[ins+offset+10+i..ins+offset+10+i+1][0];
	}
	let size1: u16 = u16::from_ne_bytes(data16);
	
	let sizes = [size0, size1];
	
	for i in 1..=0 {
		data16[i] = mmap[ins+offset+12+i..ins+offset+12+i+1][0];
	}
	let ptrs0: u16 = u16::from_ne_bytes(data16);
	
	for i in 1..=0 {
		data16[i] = mmap[ins+offset+14+i..ins+offset+14+i+1][0];
	}
	let ptrs1: u16 = u16::from_ne_bytes(data16);
	
	let ptrs = [ptrs0, ptrs1];
	
	for i in 3..=0 {
		data[i] = mmap[ins+offset+16+i..ins+offset+16+i+1][0];
	}
	let iptr: u32 = u32::from_ne_bytes(data);
	
	for i in 3..=0 {
		data[i] = mmap[ins+offset+20+i..ins+offset+20+i+1][0];
	}
	let inum: u32 = u32::from_ne_bytes(data);
	
	return Inode { refs: refs, mode: mode, size: sizes, ptrs: ptrs, iptr: iptr, inum: inum };
}

// TODO: Read from an inode...
pub fn inode_read(d: Inode, mmap: &memmap2::MmapMut) -> (Vec<u8>, u32) {
	let mut c: Vec<u8> = vec![];
	for i in 0..=d.size[0]-1 {
		c.push(mmap[ins+(d.ptrs[0] as usize)..ins+(d.ptrs[0] as usize)+1][0]);
	}
	for i in 0..=d.size[1]-1 {
		c.push(mmap[ins+(d.ptrs[1] as usize)..ins+(d.ptrs[1] as usize)+1][0]);
	}
	return (c, d.inum);	// Recursive call?
}




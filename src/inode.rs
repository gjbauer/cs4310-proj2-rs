use std::mem::size_of;

pub struct Inode {
	refs: u32,
	mode: u32,
	size: [u16; 2],
	ptrs: [u16; 2],
	iptr: u32,
	inum: u32,
}

// TODO: Implement this function...
/*fn get_inode(inum: u32) -> Inode
{
	return;
}*/

// TODO: Implement inode_deserialize...
fn inode_deserialize(mmap: &memmap2::Mmap, num: usize) -> (Inode, &memmap2::Mmap) {
	let ins: usize = 5 * 4096;	// get_root_start();
	let offset: usize = num * std::mem::size_of::<Inode>();
	
	let mut data: [u8; 4] = [0; 4];
	let mut data16: [u8; 2] = [0; 2];
	for i in 3..=0 {	// Endian: big/little
		data[i] = mmap[ins+offset+i..ins+offset+i+1][0];
	}
	
	let refs: u32 = u32::from_ne_bytes(data);
	println!("refs = {}", refs);
	
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
	
	let ind: Inode = Inode { refs: refs, mode: mode, size: sizes, ptrs: ptrs, iptr: iptr, inum: inum };
	
	return ( ind, mmap );
}




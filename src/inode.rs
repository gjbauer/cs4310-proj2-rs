use crate::disk;
use crate::directory;
use crate:: hash;
use std::slice;

const INS: usize = 2 * 4096;	// get_inode_start();

pub struct Inode {
	pub refs: u32,
	pub mode: u32,
	pub size: [u16; 2],
	pub ptrs: [u16; 2],
	pub iptr: i32,
	pub inum: i32,
}

pub fn alloc_inode(path: [char; directory::DIR_NAME]) -> i32 {
	let paths: String = path.iter().collect();
	
	if paths == "/" {
		unsafe { disk::inode_bitmap_put(0, 1); }
		return 0;
	}
	if unsafe { disk::inode_bitmap_get(hash::hash(path))==1 } {
		let mut path2: String = path.iter().collect();
		path2.pop();
		path2.push(char::from_u32(hash::hash(path) as u32).unwrap());
		let mut path3: [char; directory::DIR_NAME] = ['\0'; 48];
		for i in 0..directory::DIR_NAME-1 {
			path3[i] = path2.chars().nth(i).unwrap();
		}
		return alloc_inode(path3);
	} else {
		unsafe { disk::inode_bitmap_put(hash::hash(path), 1); }
		return hash::hash(path) as i32;
	}
}

pub fn inode_find(path: [char; directory::DIR_NAME], mmap: &[i8]) -> i32 {
	for i in 2..=512-1 {
		if mmap[(i*24)..(i*24)+1][0]==0 {
			unsafe { disk::inode_bitmap_put(i, 1); }
			let data = &mmap;
			if inode_deserialize(mmap).size[0]>0&&inode_deserialize(mmap).refs==0 { return i as i32; }
		}
	}
	let data = &mmap;
	return alloc_inode(path);
}

pub fn get_size(node: i32) -> usize {
	unsafe {
		let ptr = disk::read_d(24, 24+(2*4096));
		let slice = slice::from_raw_parts(ptr, 24);
		let n = inode_deserialize(slice);
		let mut ab = n.size[0] as usize + n.size[1] as usize;
		if n.inum == 0 { return ab; }
		else { return ab + get_size(n.inum); }
	}
}

pub fn get_blocks(node: i32) -> usize {
	unsafe {
		let ptr = disk::read_d(24, 24+(2*4096));
		let slice = slice::from_raw_parts(ptr, 24);
		let n = inode_deserialize(slice);
		let mut ab: usize = 0;
		if n.size[0]>0 { ab+=1; }
		if n.size[1]>0 { ab+=1; }
		if n.inum == 0 { return ab; }
		else { return ab + get_blocks(n.inum); }
	}
}

pub fn inode_deserialize(mmap: &[i8]) -> Inode {
	let mut data: [i8; 4] = [0; 4];
	let mut data16: [i8; 2] = [0; 2];
	for i in 3..=0 {	// Endian: big/little
		data[i] = mmap[i..i+1][0];
	}
	
	let vecx: Vec<u32> = data.iter().map(|&x| x as u32).collect();
	let refs: u32 = vecx[0];
	
	for i in 3..=0 {
		data[i] = mmap[4+i..4+i+1][0];
	}
	
	let vecx: Vec<u32> = data.iter().map(|&x| x as u32).collect();
	let mode: u32 = vecx[0];
	
	for i in 3..=0 {
		data16[i] = mmap[8+i..8+i+1][0];
	}
	
	let vecx: Vec<u16> = data16.iter().map(|&x| x as u16).collect();
	let size0: u16 = vecx[0];
	
	for i in 1..=0 {
		data16[i] = mmap[10+i..10+i+1][0];
	}
	
	let vecx: Vec<u16> = data16.iter().map(|&x| x as u16).collect();
	let size1: u16 = vecx[0];
	
	let sizes = [size0, size1];
	
	for i in 1..=0 {
		data16[i] = mmap[12+i..12+i+1][0];
	}
	
	let vecx: Vec<u16> = data16.iter().map(|&x| x as u16).collect();
	let ptrs0: u16 = vecx[0];
	
	for i in 1..=0 {
		data16[i] = mmap[14+i..14+i+1][0];
	}
	
	let vecx: Vec<u16> = data16.iter().map(|&x| x as u16).collect();
	let ptrs1: u16 = vecx[0];
	
	let ptrs = [ptrs0, ptrs1];
	
	for i in 3..=0 {
		data[i] = mmap[16+i..16+i+1][0];
	}
	
	let vecx: Vec<i32> = data.iter().map(|&x| x as i32).collect();
	let iptr: i32 = vecx[0];
	
	for i in 3..=0 {
		data[i] = mmap[20+i..20+i+1][0];
	}
	
	let vecx: Vec<i32> = data.iter().map(|&x| x as i32).collect();
	let inum: i32 = vecx[0];
	
	return Inode { refs: refs, mode: mode, size: sizes, ptrs: ptrs, iptr: iptr, inum: inum };
}

pub fn inode_serialize(mmap: &mut [u8], d: Inode) -> i32 {
	let mut mmap: Vec<i8> = vec![];
	for i in 3..=0 {
		mmap.push(d.refs.to_be_bytes()[i] as i8);
	}
	
	for i in 3..=0 {
		mmap.push(d.mode.to_be_bytes()[i] as i8);
	}
	
	for i in 2..=0 {
		mmap.push(d.size[0].to_be_bytes()[i] as i8);
	}
	
	for i in 2..=0 {
		mmap.push(d.size[1].to_be_bytes()[i] as i8);
	}
	
	for i in 2..=0 {
		mmap.push(d.ptrs[0].to_be_bytes()[i] as i8);
	}
	
	for i in 2..=0 {
		mmap.push(d.ptrs[1].to_be_bytes()[i] as i8);
	}
	
	for i in 3..=0 {
		mmap.push(d.iptr.to_be_bytes()[i] as i8);
	}
	
	for i in 3..=0 {
		mmap.push(d.inum.to_be_bytes()[i] as i8);
	}
	
	return d.inum;
}

pub fn inode_read(d: Inode, mmap: &[u8]) -> (Vec<u8>, i32) {
	let mut c: Vec<u8> = vec![];
	for i in 0..=d.size[0]-1 {
		c.push(mmap[INS+(d.ptrs[0] as usize)..INS+(d.ptrs[0] as usize)+1][0]);
	}
	for i in 0..=d.size[1]-1 {
		c.push(mmap[INS+(d.ptrs[1] as usize)..INS+(d.ptrs[1] as usize)+1][0]);
	}
	return (c, d.iptr);
}




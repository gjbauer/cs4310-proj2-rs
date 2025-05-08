use crate::directory;

const ins: usize = 2 * 4096;	// get_root_start();

pub struct Inode {
	pub refs: u32,
	pub mode: u32,
	pub size: [u16; 2],
	pub ptrs: [u16; 2],
	pub iptr: i32,
	pub inum: i32,
}

/*
int
inode_find(const char *path) {
	int* ptr = (int*)get_inode_bitmap();
	for (int i=2; i<512; i++) {
		if (*ptr==0) {
			if (get_inode(i)->size[0]>0&&get_inode(i)->refs==0) return i;
		}
	}
	return alloc_inode(path);
}

int
alloc_inode(const char *path) {
	char *hpath;
	char tpath[DIR_NAME];
	void* ibm = get_inode_bitmap();
	if (!strcmp(path, "/")) {
		bitmap_put(ibm, 0, 1);
		return 0;
	}
	if (bitmap_get(ibm, hash(path))==1) {
		return alloc_inode(extend(path));
	} else {
		bitmap_put(ibm, hash(path), 1);
		return hash(path);
	}
}

pub fn alloc_inode(path: [char; directory::DIR_NAME], mmap: &memmap2::MmapMut) -> i32 {
	let paths: String = path.iter().collect();
	if paths == "/" {
		bitmap_put(mmap, 0, 1);
		return 0;
	}
	if (bitmap_get(ibm, hash(path))==1) {
		return alloc_inode(extend(path));
	} else {
		bitmap_put(ibm, hash(path), 1);
		return hash(path);
	}
}
*/

pub fn inode_find(path: [char; directory::DIR_NAME], mmap: &memmap2::MmapMut) -> i32 {
	for i in 2..=512-1 {
		if mmap[ins+(i*24)..ins+(i*24)+1][0]==0 {
			// bitmap_put(mmap, i, 1);
			let data = &mmap;
			if inode_deserialize(mmap, i as i32).size[0]>0&&inode_deserialize(mmap, i as i32).refs==0 { return i as i32; }
		}
	}
	let data = &mmap;
	return alloc_inode(path, data);
}

pub fn inode_deserialize(mmap: &memmap2::MmapMut, num: i32) -> Inode {
	let offset: usize = (num as usize) * std::mem::size_of::<Inode>();
	
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
	let iptr: i32 = i32::from_ne_bytes(data);
	
	for i in 3..=0 {
		data[i] = mmap[ins+offset+20+i..ins+offset+20+i+1][0];
	}
	let inum: i32 = i32::from_ne_bytes(data);
	
	return Inode { refs: refs, mode: mode, size: sizes, ptrs: ptrs, iptr: iptr, inum: inum };
}

pub fn inode_serialize(mmap: &mut memmap2::MmapMut, d: Inode) -> i32 {
	let offset: usize = (d.inum as usize) * std::mem::size_of::<Inode>();
	
	for i in 3..=0 {
		mmap[ins+offset+i..ins+offset+i+1][0] = d.refs.to_be_bytes()[i];
	}
	
	for i in 3..=0 {
		mmap[ins+offset+4+i..ins+offset+4+i+1][0] = d.mode.to_be_bytes()[i];
	}
	
	for i in 2..=0 {
		mmap[ins+offset+8+i..ins+offset+8+i+1][0] = d.size[0].to_be_bytes()[i];
	}
	
	for i in 2..=0 {
		mmap[ins+offset+10+i..ins+offset+10+i+1][0] = d.size[1].to_be_bytes()[i];
	}
	
	for i in 2..=0 {
		mmap[ins+offset+12+i..ins+offset+12+i+1][0] = d.ptrs[0].to_be_bytes()[i];
	}
	
	for i in 2..=0 {
		mmap[ins+offset+14+i..ins+offset+14+i+1][0] = d.ptrs[1].to_be_bytes()[i];
	}
	
	for i in 3..=0 {
		mmap[ins+offset+16+i..ins+offset+16+i+1][0] = d.iptr.to_be_bytes()[i];
	}
	
	for i in 3..=0 {
		mmap[ins+offset+20+i..ins+offset+20+i+1][0] = d.inum.to_be_bytes()[i];
	}
	
	mmap.flush().expect("ERROR.");
	
	return d.inum;
}

pub fn inode_read(d: Inode, mmap: &memmap2::MmapMut) -> (Vec<u8>, i32) {
	let mut c: Vec<u8> = vec![];
	for i in 0..=d.size[0]-1 {
		c.push(mmap[ins+(d.ptrs[0] as usize)..ins+(d.ptrs[0] as usize)+1][0]);
	}
	for i in 0..=d.size[1]-1 {
		c.push(mmap[ins+(d.ptrs[1] as usize)..ins+(d.ptrs[1] as usize)+1][0]);
	}
	return (c, d.iptr);
}




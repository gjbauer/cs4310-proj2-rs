use crate::inode;

pub const DATA_START:usize = 5 * 4096;	// get_root_start();
pub const DIR_NAME: usize = 48;

pub struct Dirent {
	pub name: [char; DIR_NAME],
	pub inum: u32,
	pub active: bool,
}

/*
pub fn readdir(mmap: &memmap2::MmapMut, path: [char; DIR_NAME]) -> Vec<Dirent> {
}*/

pub fn tree_lookup(mmap: &memmap2::MmapMut,path: [char; DIR_NAME]) -> i32 {
	let paths: String = path.iter().collect();
	if paths.len()==1 { return 0; }
	let pathv: Vec<&str> =paths.split('/').collect();
	let mut l: i32 = 0;
	
	for i in 0..=pathv.len()-1 {
		let mut cpath: String = "/".to_string();
		for j in 0..=i {
			cpath.push_str(&pathv[j]);
			cpath.push('/');
		}
		cpath.pop();

		let data = &mmap;
		let n = inode::inode_deserialize(data, l);
	
		let data = &mmap;
		let p0 = dirent_deserialize(data, n.ptrs[0] as usize);
		let mut nm0: String = "".to_string();
		for i in 0..DIR_NAME-1 { nm0.push(p0.name[i]); }
		if nm0 == cpath {
			if nm0==paths { return p0.inum as i32; }
			l = p0.inum as i32;
		}
	
		let data = &mmap;
		let p1 = dirent_deserialize(data, n.ptrs[1] as usize);
		let mut nm1: String = "".to_string();
		for i in 0..DIR_NAME-1 { nm1.push(p1.name[i]); }
		if nm1 == cpath { 
			if nm1==paths { return p1.inum as i32; }
			l = p1.inum as i32;
		}
		
		if n.iptr == 0 { return -2; }
		else { l = n.iptr; }
	}
	
	return -2;
}

pub fn dirent_deserialize(mmap: &memmap2::MmapMut, offset: usize) -> Dirent {
	let mut name: [char; DIR_NAME] = ['\0'; 48];
	
	for i in 0..=DIR_NAME-1 {
		let data = mmap[DATA_START+offset+i..DATA_START+offset+i+1][0];
		name[i] = data as char;
	}
	
	let data = mmap[DATA_START+offset+49..DATA_START+offset+50][0];
	let inum = data as u32;
	
	let data = mmap[DATA_START+offset+51..DATA_START+offset+52][0];
	let active = data != 0;
	
	return Dirent { name: name, inum: inum, active: active } ;
}

pub fn dirent_serialize(mut mmap: memmap2::MmapMut, offset: usize, ent: &Dirent) -> memmap2::MmapMut {
	let name: [char; DIR_NAME] = ['\0'; 48];
	
	for i in 0..=DIR_NAME-1 {
		for j in 3..=0 {
			mmap[DATA_START+offset+(i*4)+j..DATA_START+offset+(i*4)+j+1][0] = ent.name[i].encode_utf8(&mut [0; DIR_NAME]).as_bytes()[j];
		}
	}
	
	for i in 3..=0 {
		mmap[DATA_START+offset+(DIR_NAME*4)+i..DATA_START+offset+(DIR_NAME*4)+i+1][0] = ent.inum.to_le_bytes()[i];
	}
	
	mmap.flush_async().expect("ERROR.");
	
	mmap[DATA_START+offset+(DIR_NAME*4)+3..DATA_START+offset+(DIR_NAME*4)+4][0] = ent.active as u8;
	
	return mmap ;
}
/*
pub fn mknod(mmap: &memmap2::MmapMut,path: [char; DIR_NAME], mode: u32) -> i32 {
	//while true {
	//}
}
*/

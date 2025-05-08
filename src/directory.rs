use crate::inode;

const data_start:usize = 5 * 4096;	// get_root_start();
pub const DIR_NAME: usize = 48;

pub struct Dirent {
	pub name: [char; DIR_NAME],
	pub inum: u32,
	pub active: bool,
}

pub fn tree_lookup(mmap: &memmap2::MmapMut,path: [char; DIR_NAME], mut l: i32) -> i32 {
	if path[1] == '\0' { return 0; }
	let paths: String = path.iter().collect();
	let pathv: Vec<&str> =paths.split('/').collect();
	
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
		let data = mmap[data_start+offset+i..data_start+offset+i+1][0];
		name[i] = data as char;
	}
	
	let data = mmap[data_start+offset+49..data_start+offset+50][0];
	let inum = data as u32;
	
	let data = mmap[data_start+offset+51..data_start+offset+52][0];
	let active = data != 0;
	
	return Dirent { name: name, inum: inum, active: active } ;
}

pub fn dirent_serialize(mmap: &mut memmap2::MmapMut, offset: usize, ent: Dirent) -> u32 {
	let mut name: [char; DIR_NAME] = ['\0'; 48];
	
	for i in 0..=DIR_NAME-1 {
		for j in 3..=0 {
			mmap[data_start+offset+(i*4)+j..data_start+offset+(i*4)+j+1][0] = ent.name[i].encode_utf8(&mut [0; DIR_NAME]).as_bytes()[j];
		}
	}
	
	for i in 3..=0 {
		mmap[data_start+offset+(DIR_NAME*4)+i..data_start+offset+(DIR_NAME*4)+i+1][0] = ent.inum.to_be_bytes()[i];
	}
	
	mmap[data_start+offset+(DIR_NAME*4)+3..data_start+offset+(DIR_NAME*4)+4][0] = ent.active as u8;
	
	return ent.inum as u32 ;
}
/*
pub fn mknod(ent: Dirent, mode: u32) -> u32 {
}
*/

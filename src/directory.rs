use crate::inode;
use crate::disk;

pub const DATA_START:usize = 5 * 4096;	// get_root_start();
pub const DIR_NAME: usize = 48;

pub struct Dirent {
	pub name: [char; DIR_NAME],
	pub inum: i32,
	pub active: bool,
}

/*
pub fn readdir(mmap: &memmap2::MmapMut, path: [char; DIR_NAME]) -> Vec<Dirent> {
}*/

pub fn tree_lookup(mmap: &[i8],path: [char; DIR_NAME]) -> (i32, i32) {
	let paths: String = path.iter().collect();
	if paths.len()==1 { return (0, 0); }
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
		let mut buf: [i8; 52] = [42; 52];
		unsafe {
		for i in 0..=51 {
			buf[i] = *disk::read_d( 1 , DATA_START + i + n.ptrs[0] as usize).as_ref().unwrap();
		}
		}
		let p0 = dirent_deserialize(&buf);
		let mut nm0: String = "".to_string();
		for i in 0..DIR_NAME-1 { nm0.push(p0.name[i]); }
		if nm0 == cpath {
			if nm0==paths { return (0, p0.inum); }
			l = p0.inum as i32;
		}
	
		let data = &mmap;
		let mut buf: [i8; 52] = [42; 52];
		unsafe {
		for i in 0..=51 {
			buf[i] = *disk::read_d( 1 , DATA_START + i + n.ptrs[1] as usize).as_ref().unwrap();
		}
		}
		let p1 = dirent_deserialize(&buf);
		let mut nm1: String = "".to_string();
		for i in 0..DIR_NAME-1 { nm1.push(p1.name[i]); }
		if nm1 == cpath { 
			if nm1==paths { return (0, p1.inum); }
			l = p1.inum;
		}
		
		if n.iptr == 0 { return (-2, l); }
		else { l = n.iptr; }
	}
	
	return (-2, l);
}

pub fn rename(ent: Dirent, to: [char; DIR_NAME] ) -> Dirent {
	let mut rnm: Dirent = Dirent { name: [0 as char; DIR_NAME], inum: ent.inum, active: ent.active };
	for i in 0..=DIR_NAME-1 {
		rnm.name[i] = to[i];
	}
	return rnm;
}

pub fn dirent_deserialize(mmap: &[i8]) -> Dirent {
	let mut name: [char; DIR_NAME] = ['\0'; 48];
	
	for i in 0..=DIR_NAME-1 {
		let data = mmap[i..i+1][0];
		name[i] = data as u8 as char;
	}
	
	let data = mmap[49..50][0];
	let inum = data as i32;
	
	let data = mmap[51..52][0];
	let active = data != 0;
	
	return Dirent { name: name, inum: inum, active: active } ;
}

pub fn dirent_serialize(ent: &Dirent) -> Vec<i8> {
	let name: [char; DIR_NAME] = ['\0'; 48];
	let mut mvec: Vec<i8> = vec![];
	
	for i in 0..=DIR_NAME-1 {
		mvec.push(ent.name[i].encode_utf8(&mut [0; DIR_NAME]).as_bytes()[0] as i8);
	}
	
	for i in 3..=0 {
		mvec.push(ent.inum.to_le_bytes()[i] as i8);
	}
	
	mvec.push(0);
	
	mvec.push(ent.active as i8);
	
	return mvec ;
}

pub fn mknod(mmap: &[i8], path: [char; DIR_NAME], mode: u32) -> i32 {
	let ret = tree_lookup(mmap, path);
	let rv = ret.0;
	let l = ret.1;
	//while true {
	//}
}


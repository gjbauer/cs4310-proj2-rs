const data_start:usize = 5 * 4096;	// get_root_start();
pub const DIR_NAME: usize = 48;

pub struct Dirent {
	pub name: [char; DIR_NAME],
	pub inum: u32,
	pub active: bool,
}

// TODO: Implement dirent_deserialize...
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

// TODO: Implement dirent_serialize...
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


const data_start:usize = 5 * 4096;	// get_root_start();
pub const DIR_NAME: usize = 48;

pub struct Dirent {
	pub name: [char; DIR_NAME],
	pub inum: u16,
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
	let inum = data as u16;
	
	let data = mmap[data_start+offset+51..data_start+offset+52][0];
	let active = data != 0;
	
	return Dirent { name: name, inum: inum, active: active } ;
}

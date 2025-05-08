use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::io::{self, Write};
mod directory;

pub fn storage_init(name: [char; directory::DIR_NAME]) -> memmap2::MmapMut {
	let file_path = "example.dat";
    
	let file = OpenOptions::new()
		.read(true)
		.write(true)
		.open(file_path)?;
	let mut mmap = unsafe { MmapMut::map_mut(&file)? };
	
	return mmap;
}

/*
 * pub fn read_fs() {
 * }
 *
 * pub fn write_fs() {
 * }
 */

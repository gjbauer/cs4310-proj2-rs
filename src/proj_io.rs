use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::io::{self, Write};
use crate::directory;

// TODO: mount argv[2] as data file
pub fn storage_init(name: [char; directory::DIR_NAME]) -> Option<memmap2::MmapMut> {
	let filename: String = name.iter().collect();
	let file = OpenOptions::new()
                       .read(true)
                       .write(true)
                       .create(true)
                       .open(filename).ok()?;
        
	let mut mmap: Option<memmap2::MmapMut> = unsafe { Some(MmapMut::map_mut(&file).ok()?) };
	
	return mmap;
}

/*
 * pub fn read_fs() {
 * }
 *
 * pub fn write_fs() {
 * }
 */

extern crate fuse;

use std::env;
use fuse::Filesystem;

struct Nufs;

impl Filesystem for Nufs {
	// TODO: Put prototype functions here...
}

fn main() {
	let mountpoint = match env::args().nth(1) {
		Some(path) => path,
		None => {
			println!("Usage: {} <MOUNTPOINT>", env::args().nth(0).unwrap());
			return;
		}
	};
	
	// Open the file
	/*let file = OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.open("data.nufs")?;
	
	// TODO: Create a memory map for the file
	let mmap: memmap2::MmapMut = unsafe {  };*/
	
	fuse::mount(Nufs, &mountpoint, &[]);
}

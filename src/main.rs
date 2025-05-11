extern crate fuse;
extern crate libc;
extern crate time;

use std::env;
use std::path::Path;
use std::slice;
use libc::{ENOENT, ENOSYS};
use fuse::{FileAttr, FileType, Filesystem, Request, ReplyAttr, ReplyData, ReplyEntry, ReplyDirectory};
mod inode;
mod directory;
mod disk;
mod hash;

struct Nufs;

impl Filesystem for Nufs {
	fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
		println!("getattr(ino={})", ino);
		reply.error(ENOSYS);
	}
}

fn main() {
	let mountpoint = match env::args().nth(1) {
		Some(path) => path,
		None => {
			println!("Usage: {} <MOUNTPOINT>", env::args().nth(0).unwrap());
			return;
		}
	};
	
	unsafe {
		disk::storage_init();
		let ptr = disk::read_d(52, 52+(5*4096));
		let slice = slice::from_raw_parts(ptr, 52+(5*4096));
		let mut d = directory::dirent_deserialize(slice);
		let name: String = d.name.iter().collect();
		println!("{}", name);
		d.name[1] = 'h';
		let mut d = directory::dirent_serialize(&d);
		let c: &mut [i8] = &mut d;
		disk::write_d(c.as_mut_ptr(), 52, 52+(5*4096));
	}
	
	
	fuse::mount(Nufs, &mountpoint, &[]);
}

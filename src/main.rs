extern crate fuse;
extern crate libc;

use std::env;
use std::path::Path;
use std::slice;
use libc::{ENOENT, ENOSYS};
use std::ffi::OsStr;
use fuse::{FileAttr, FileType, Filesystem, Request, ReplyAttr, ReplyData, ReplyEntry, ReplyDirectory};
mod inode;
mod directory;
mod disk;
mod hash;

struct Nufs;

impl Filesystem for Nufs {
	fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
		unsafe {
			let mut buf: [i8; 24] = [42; 24];
			for i in 0..23 {
				buf[i] = disk::read_d(1, ((2*4096)+(ino as usize*24)+i)) as i8;
			}
			
			inode::inode_deserialize(&buf, ino as i32);
			println!("getattr(ino={})", ino);
			reply.error(ENOSYS);
		}
	}
	fn readdir(&mut self, _req: &Request, ino: u64, fh: u64, offset: i64, mut reply: ReplyDirectory) {
		println!("readdir(ino={}, fh={}, offset={})", ino, fh, offset);
		reply.error(ENOSYS);
	}
	fn mknod(&mut self, _req: &Request<'_>, _parent: u64, _name: &OsStr, _mode: u32, _rdev: u32, reply: ReplyEntry,) {
		println!("mknod({:?})", _name);
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
		let slice = slice::from_raw_parts(ptr, 52);
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

use fuser::{Filesystem, Request, ReplyAttr, ReplyData, ReplyDirectory};
use std::collections::HashMap;
use std::ffi::CString;
use std::str;
use std::slice;
mod directory;
mod inode;
mod disk;
mod hash;

struct SimpleFilesystem {
	// Here, you would store your filesystem data, e.g., a map of paths to file attributes
	root_dir: HashMap<String, directory::Dirent>,
	// Add other necessary fields for your filesystem implementation
}

impl Filesystem for SimpleFilesystem {
	/*fn init(&mut self, _session: &mut Session) -> Result<()> {
		// Perform any initialization tasks here
		Ok(())
	}*/

	fn getattr(&mut self, _req: &Request, ino: u64, _fh: Option<u64>, reply: ReplyAttr) {
		// Get the attributes of a file or directory
		/*if let Some(attr) = self.root_dir.get(path) {
			return Ok(attr.file_attr.clone());
		}

		Err(Error::ENOENT) // Not found*/
	}

	fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, reply: ReplyDirectory) {
		// List the contents of a directory
		//if  == Path::new("/") {	// _path.as_ref() <- old
			let entries: Vec<directory::Dirent> = Vec::new();
			// Example: Add the root directory entry
			//entries.push((Path::new("/"), self.root_dir[&Path::new("/")].clone()));
			// Add other entries as needed
			return ();
		//}
		//Ok(-1) // Not found
	}

	// Implement other necessary FUSE operations like read, write, etc.
	// Example: Implement read
	fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, _size: u32, _flags: i32, _lock: Option<u64>, reply: ReplyData) {
		// Implementation for reading file data
		/*if let Some(attr) = self.root_dir.get(path) {
			if attr.file_attr.file_type == FileType::File {
				// Replace this with your actual file read logic
				Ok(b"Hello, world!".to_vec())
			} else {
				Err(Error::ENOTDIR) // Not a file
			}
		} else {
			Err(Error::ENOENT) // Not found
		}*/
	}
}

fn main() -> std::io::Result<()> {
	// Open the file
	/*let file = OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.open("data.nufs")?;
	
	// TODO: Create a memory map for the file
	let mut mmap: memmap2::MmapMut = unsafe { MmapMut::map_mut(&file)? };*/
	
	let start:usize = 5 * 4096;	// get_root_start();
	/*
	let data = &mmap[start..start+1]; // Read the first byte of root
	if let Ok(_) = std::str::from_utf8(data) {
		println!("First entry: {}", std::str::from_utf8(data).unwrap());
	} else {
		println!("ERROR: No Data!!");
	}
	
	let data = &mmap[start+52..start+10+52]; // /hello.txt
	if let Ok(_) = std::str::from_utf8(data) {
		println!("Second entry: {}", std::str::from_utf8(data).unwrap());
	} else {
		println!("ERROR: No Data!!");
	}
	*/
	
	unsafe {
		disk::storage_init();
		let ptr = disk::read_d(52, 52);
		let slice = slice::from_raw_parts(ptr, 52);
		let mut d = directory::dirent_deserialize(slice);
		let name: String = d.name.iter().collect();
		println!("{}", name);
		d.name[1] = 'd';
		let mut d = directory::dirent_serialize(&d);
		let c: &mut [i8] = &mut d;
		disk::write_d(c.as_mut_ptr(), 52, 52);
	}
	
	/*println!("Size of struct: {} bytes", size_of::<directory::Dirent>());
	
	let data = &mmap;
	
	println!("Dirent name: {}", directory::dirent_deserialize(data, 0).name.iter().collect::<String>());
	
	let data = &mmap;
	println!("refs = {}", inode::inode_deserialize(data, 0).refs);
	
	let name = directory::dirent_deserialize(data, 0).name;
	//let data = &mmap;
	let d = directory::Dirent { name: name, inum: 5, active: false };
	mmap = directory::dirent_serialize(mmap, 0, &d);
	
	mmap.flush_async()?;
	
	let data = &mmap[directory::data_start+49..directory::data_start+50][0];
	println!("Dirent inum: {}", data);*/
	
	//let data = &mmap[ins+offset+51..ins+offset+52];
	//let active = data[0] != 0;
	
	//let (head, body, _tail) = unsafe { &mmap[start+72..start+72+72].align_to::<directory::Dirent>() }; // /hello.txt
	//assert!(head.is_empty(), "Data was not aligned");
	//let data: directory::Dirent = body[0];
	

	// Get the mount point from the command line arguments
	//let mount_point = std::env::args().nth(1).expect("Usage: <program_name> <mount_point>");

	// Create a new filesystem instance
	//let filesystem = SimpleFilesystem { root_dir: HashMap::new() };
	//let filesystem = SimpleFilesystem {};
	
	// Mount the filesystem
	//fuser::mount2(filesystem, mount_point, &[MountOption::AutoUnmount]).unwrap();
	
	Ok(())
}


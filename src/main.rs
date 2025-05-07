use std::collections::HashMap;
use fuser::{Filesystem, MountOption, Request, Session, ReplyAttr, ReplyData, ReplyDirectory, FileAttr, FileType};
use std::env;
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;
mod directory;
mod inode;
use std::mem::size_of;

struct SimpleFilesystem {
	// Here, you would store your filesystem data, e.g., a map of paths to file attributes
	//root_dir: HashMap<Path, directory::Dirent>,
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

	fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
		// List the contents of a directory
		//if  == Path::new("/") {	// _path.as_ref() <- old
			let mut entries: Vec<directory::Dirent> = Vec::new();
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
	let file = File::open("data.nufs")?;

	// Create a memory map for the file
	let mmap = unsafe { Mmap::map(&file)? };

	// Access file content as a byte slice
	let content = &mmap[..];

	// Print it to the console
	println!("File content: {}", String::from_utf8_lossy(content));
	
	let start:usize = 5 * 4096;	// get_root_start();
	
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
	
	println!("Size of struct: {} bytes", size_of::<directory::Dirent>());
	
	println!("Dirent name: {}", directory::dirent_deserialize(mmap, 0).name.iter().collect::<String>());
	
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


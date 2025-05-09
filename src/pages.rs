use std::os::raw::c_int;

#[unsafe(no_mangle)]
unsafe extern "C" {
	//pub fn pages_init(path: [u8]);
	
	//pub fn pages_free();
	
	//pub fn pages_get_page(_: u16) -> c_int;
	
	//pub fn get_pages_bitmap(_: u16) -> c_int;
	
	//pub fn get_inode_bitmap(_: u16) -> c_int;
	
	//pub fn get_inode_start(_: u16) -> c_int;
	
	//pub fn get_root_start(_: u16) -> c_int;
	
	pub fn hello();
}


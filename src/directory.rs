const DIR_NAME: usize = 48;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dirent {
	name: [char; DIR_NAME],
	inum: u16,
}

pub const DIR_NAME: usize = 48;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dirent {
	pub name: [char; DIR_NAME],
	pub inum: u16,
}

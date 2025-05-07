const DIR_NAME: usize = 48;

pub struct Dirent {
	name: [char; DIR_NAME],
	inum: u16,
}

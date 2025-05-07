const DIR_NAME: usize = 48;

struct Dirent {
	name: [char; DIR_NAME],
	inum: u16,
}

use arrayvec::ArrayString;
const DIR_NAME: usize = 48;

struct Dirent {
	name: ArrayString<DIR_NAME>,
	inum: u16,
}

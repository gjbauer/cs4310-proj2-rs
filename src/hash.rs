use crate::directory;

pub fn hash(name: [char; directory::DIR_NAME]) -> usize {
	let mut hash = 5381;
	let mut c: usize;
	
	for i in 0..=directory::DIR_NAME-1 {
		c = name[i] as usize;
		hash = ((hash << 5) + hash) + c; /* hash * 33 + c */
	}
	
	return hash;
}

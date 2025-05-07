pub struct Inode {
	refs: u32,
	mode: u32,
	size: [u16; 2],
	ptrs: [u16; 2],
	iptr: u32,
	inum: u32,
}

// TODO: Implement this function...
/*fn get_inode(inum: u32) -> Inode
{
	return;
}*/

//int inode_find(const char *path);
//int alloc_inode(const char *path);


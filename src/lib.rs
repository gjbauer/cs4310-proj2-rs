mod inode;
mod directory;
mod disk;
mod hash;
// TODO: Implement all of the functions as you would in C in Rust, and then call them from another C layer which can interact directly with FUSE

// implementation for: man 2 access
// Checks if a file exists.
#[unsafe(no_mangle)]
extern "C" fn ufs_access(path: [char; directory::DIR_NAME], mask: i32)
{
}

unsafe extern "C" {
	pub fn ufs_access(path: [char; directory::DIR_NAME], mask: i32);
}

// mknod makes a filesystem object like a file or directory
// called for: man 2 open, man 2 link
#[unsafe(no_mangle)]
pub fn ufs_mknod(path: [char; directory::DIR_NAME], mode: i32, rdev: i32)
{
}

// most of the following callbacks implement
// another system call; see section 2 of the manual
#[unsafe(no_mangle)]
pub fn ufs_mkdir(path: [char; directory::DIR_NAME], mode: i32, rdev: i32)
{
	// TODO: Nested Directories
}

#[unsafe(no_mangle)]
pub fn ufs_create(path: [char; directory::DIR_NAME], mode: i32)
{
	/*
	if (nufs_mknod(path, mode, 0)) {
    		int l = tree_lookup(path);
    		inode *n = get_inode(l);
        	n->mode = mode; // regular file
        	n->size = 0;
        	return l;
	} else return -1;
	 */
}

#[unsafe(no_mangle)]
pub fn ufs_unlink(path: [char; directory::DIR_NAME])
{
	let mut rv: i32 = 0;
	let mut pathv: Vec<char> = vec![];
	for i in 0..=directory::DIR_NAME-1 {
		pathv.push(path[i]);
	}
	let paths: String = pathv.into_iter().collect();
	println!("unlink({}) -> {}\n", paths, rv);
}

#[unsafe(no_mangle)]
pub fn ufs_link(from: [char; directory::DIR_NAME], to: [char; directory::DIR_NAME])
{
	let mut rv: i32 = 0;
	let mut fromv: Vec<char> = vec![];
	for i in 0..=directory::DIR_NAME-1 {
		fromv.push(from[i]);
	}
	let mut tov: Vec<char> = vec![];
	for i in 0..=directory::DIR_NAME-1 {
		tov.push(to[i]);
	}
	let froms: String = fromv.into_iter().collect();
	let tos: String = tov.into_iter().collect();
	println!("link({} => {}) -> {}\n", froms, tos, rv);
	return rv;
}

/*
// implementation for: man 2 stat
// gets an object's attributes (type, permissions, size, etc)
int
nufs_getattr(const char *path, struct stat *st)
// What I hate about this is how it will now create a file for each one that is tests exists...not very great of average UX
{
    int rv = 0;
    if (st) printf("getattr(%s) -> (%d) {mode: %04o, size: %ld}\n", path, rv, st->st_mode, st->st_size);
    else printf("getattr(%s) -> (%d)\n", path, rv);
    return rv;
}	// <- keep getattr in C?

// implementation for: man 2 readdir
// lists the contents of a directory
int
nufs_readdir(const char *path, void *buf, fuse_fill_dir_t filler,
             off_t offset, struct fuse_file_info *fi)
{
    printf("readdir(%s) -> %d\n", path, rv);
    return 0;
}

int
nufs_rmdir(const char *path)
{
    int rv = -1;
    rv = nufs_unlink(path);
    printf("rmdir(%s) -> %d\n", path, rv);
    return rv;
}

// implements: man 2 rename
// called to move a file within the same filesystem
int
nufs_rename(const char *from, const char *to) {
    printf("rename(%s => %s) -> %d\n", from, to, rv);
    return rv;
}

int
nufs_chmod(const char *path, mode_t mode)
{
    int rv = -1;
    printf("chmod(%s, %04o) -> %d\n", path, mode, rv);
    return rv;
}

int
nufs_truncate(const char *path, off_t size)
{
    int rv = 0;
    //int l = tree_lookup(path
    printf("truncate(%s, %ld bytes) -> %d\n", path, size, rv);
    return rv;
}

// this is called on open, but doesn't need to do much
// since FUSE doesn't assume you maintain state for
// open files.
int
nufs_open(const char *path, struct fuse_file_info *fi)
{
    int rv = 0;
    int k = nufs_access(path, 0);
    if (k==ENOENT) k = nufs_create(path, 0100644, 0);
    printf("open(%s) -> %d\n", path, rv);
    return rv;
}

// Actually read data
int
nufs_read(const char *path, char *buf, size_t size, off_t offset, struct fuse_file_info *fi)
{
    printf("read(%s, %ld bytes, @+%ld) -> %d\n", path, size, offset, rv);
    return rv;
}

// Actually write data
int
nufs_write(const char *path, const char *buf, size_t size, off_t offset, struct fuse_file_info *fi)
{
    printf("write(%s, %ld bytes, @+%ld) -> %d\n", path, size, offset, rv);
    return rv;
}
 */


/*
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]	// <- important
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/

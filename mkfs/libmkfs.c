#include <string.h>
#include <stdint.h>
#include "pages.h"
#include "inode.h"
#include "bitmap.h"
#include "directory.h"
#include "mkfs.h"
#include "inode.h"
#include "libmkfs.h"
#include <stdlib.h>
#include <stdbool.h>
#include <errno.h>

int split(const char *path, int n, char buf[DIR_NAME]) {
	int rv=0;
	if (n==0) {
		strcpy(buf, "/");
	} else {
		int c=0, i=0;
		for (; path[i] && c<n+1; i++) {
			buf[i]=path[i];
			if (path[i]=='/') c++;
		}
		if (buf[i-1]=='/') buf[i-1]='\0';
	}
	return rv;
}

bool is_empty(inode *n)
{
	return ( n->size[0]==0 || n->size[1]==0 ) ? (true) : (false) ;
}

int
inode_size(inode *n)
{
	return (n->size[0]+n->size[1]);
}

char*
get_data_end()
{
	inode *h=get_inode(1);
	return (char*)get_root_start()+h->ptrs[0];
}

char*
get_data(int offset)
{
	return (char*)get_root_start()+offset;
}

int
write_sp(char *data, int inode, int ptr, const char *buf, size_t size)
{
	struct inode n; // *get_inode(inode);
	memcpy(&n, get_inode(inode), sizeof(n));
	struct inode h; // *get_inode(1);
	memcpy(&h, get_inode(1), sizeof(n));
	memcpy(data, buf, size);
	data[size] = '\0';
	n.size[ptr]=size;
	n.ptrs[ptr] = h.ptrs[0];
	h.ptrs[0] += size;
	memcpy(get_inode(inode), &n, sizeof(n));
	memcpy(get_inode(1), &h, sizeof(h));
}

int
_remainder(inode *n, int size, int offset)
{
	return ( size - ( ( n->size[0] + n->size[1] ) - offset ) );
}

int
_write(const char *path, const char *buf, size_t size, off_t offset, int l)
{
	int rv = 0;
	(l == 0) ? l = tree_lookup(path, find_parent(path)) : l;
	inode *n = get_inode(l), *h = get_inode(1);
	
	int s = inode_size(n);
	
	if (s == 0) write_sp(get_data_end()+offset, l, 0, buf, size);
	else if (is_empty(n)) write_sp(get_data_end()+offset-s, l, 1, buf, size);
	else {
		int r = _remainder(n, size, offset);
		if (r<=0) {
			if (offset < n->size[0]) {
				write_sp(get_data(n->ptrs[0]+offset), l, 0, buf, n->size[0]-offset);
				size-=n->size[0];
			}
			if (size > 0) {
				write_sp(get_data(n->ptrs[1]+(offset-n->size[0])), l, 1, buf, size );
			}
		}
		else {
			if (n->iptr == 0) n->iptr = inode_find(path);
			if (_remainder(n, size, offset) >= size) {
				return _write(path, buf, (size), (_remainder(n, size, offset) - size), n->iptr);
			}
			else
				return _write(path, buf, (size - _remainder(n, size, offset)), (0), n->iptr);
		}
	}
	printf("write(%s, %ld bytes, @+%ld) -> %d\n", path, size, offset, rv);
	return rv;
}

int
write(const char *path, const char *buf, size_t size, off_t offset)
{
	return _write(path, buf, size, offset, 0);
}

int
count_l(const char *path) {
	int c=0;
	for(int i=0; path[i]; i++) {
		if (path[i]=='/') c++;
	}
	return c;
}

int
find_parent(const char *path)
{
	char trm[DIR_NAME];
	int i;
	for(i=0; path[i]; i++) trm[i]=path[i];
	if (trm[i]=='/') trm[i]='\0';
	char ptr[DIR_NAME];
	int k = count_l(trm);
	int n=0;
	for (int i=0; i<k; i++) {
		split(trm, i, ptr);
		n = tree_lookup(ptr, n);
		if (n<0) return -ENOENT;
	}
	return n;
}

char*
parent_path(const char *path)
{
	char trm[DIR_NAME];
	int i;
	for(i=0; path[i]; i++) trm[i]=path[i];
	if (trm[i]=='/') trm[i]='\0';
	char ptr[DIR_NAME];
	int k = count_l(trm);
	int n=0;
	for (int i=0; i<k; i++) {
		split(trm, i, ptr);
	}
	
	char *m = (char*)malloc(DIR_NAME * sizeof(char));
	strncpy(m, ptr, DIR_NAME);
	return m;
}

int
count_placement(inode *d, const char* path, const char *ppath)
{
	int i=0;
	dirent *e;
	while (true) {
		e = (dirent*)get_data(d->ptrs[0]);
		if (!strcmp(e->name, "") || ( d->size[0]==0 ) ) break;
		i++;
		e = (dirent*)get_data(d->ptrs[1]);
		if (!strcmp(e->name, "") || (d->size[1]==0) ) break;
		i++;
		d = (d == 0) ? get_inode( (d->iptr = inode_find(ppath)) ) : get_inode(d->iptr);
	}
	return i;
}

int
mknod(const char *path, int mode)
{
	int rv = 0;
	int l = (!strcmp(path, "/")) ? 0 : inode_find(path);
	char *ppath = parent_path(path);
	
	dirent e;
	inode *h = get_inode(tree_lookup(ppath, find_parent(ppath)));
	inode *n = get_inode(l);
	n->mode=mode;
	n->refs = 1;
	
	strncpy(e.name, path, DIR_NAME);
	e.inum = l;
	e.active = true;
	
	int i = count_placement(h, path, ppath);
	
	write(ppath, (char*)&e, sizeof(dirent), i*sizeof(dirent));
	

	free(ppath);
	printf("mknod(%s) -> %d\n", path, rv);
	return rv;
}

// most of the following callbacks implement
// another system call; see section 2 of the manual
int
mkdir(const char *path, int mode)
{
	int rv = mknod(path, mode | 040000);
	printf("mkdir(%s) -> %d\n", path, rv);
	return rv;
}

void
mkfs() {
	pages_init("../data.nufs");
	mkdir("/", 755);
	mknod("/hello.txt", 755);
	pages_free();
}


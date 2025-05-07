#include "inode.h"
#include "directory.h"
#include "hash.h"
#include <string.h>
#include <stdlib.h>
#include <errno.h>

int tree_lookup(const char* subpath, int i) {
	if (!strcmp(subpath, "/")) return 0;
	inode *n = get_inode(i);
	dirent *p0, *p1;
lookup_loop:
	p0 = (dirent*)((char*)get_root_start()+n->ptrs[0]);
	p1 = (dirent*)((char*)get_root_start()+n->ptrs[1]);
	if (!strcmp(p0->name, subpath)) {
		return p0->inum;
	} else if (n->ptrs[0]==0) {
		return -ENOENT;
	} else if (!strcmp(p1->name, subpath)) {
		return p1->inum;
	} else if (n->ptrs[1]==0) {
		return -ENOENT;
	} else if (n->iptr!=0) {
		n = get_inode(n->iptr);
		goto lookup_loop;
	}
	return -ENOENT;
}



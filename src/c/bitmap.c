#include "bitmap.h"

size_t bitmap_get(void* bm, size_t ii) {
	size_t* ptr = (size_t*)get_inode_bitmap();
	size_t os = ii / sizeof(size_t);
	ptr =  ptr + os;
	return *ptr;
}

void bitmap_put(void* bm, size_t ii, size_t vv) {
	size_t* ptr = (size_t*)get_inode_bitmap();
	size_t os = ii / sizeof(size_t);
	ptr =  ptr + os;
	*ptr = vv;
}


#include "bitmap.h"

int _bitmap_get(void* bm, size_t ii) {
	size_t* ptr = (size_t*)bm;
	size_t os = ii / sizeof(size_t);
	ptr =  ptr + os;
	return *ptr;
}

int bitmap_get(size_t ii) {
	return _bitmap_get(get_inode_bitmap(), ii);
}

void _bitmap_put(void* bm, size_t ii, size_t vv) {
	size_t* ptr = (size_t*)bm;
	size_t os = ii / sizeof(size_t);
	ptr =  ptr + os;
	*ptr = vv;
}

void bitmap_put(size_t ii, size_t vv) {
	return _bitmap_get(get_inode_bitmap(), ii, vv);
}

void bitmap_print(void* bm, int size) {
	return;
}


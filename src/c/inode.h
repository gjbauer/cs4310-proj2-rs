// based on cs3650 starter code

#ifndef INODE_H
#define INODE_H

#include <stdint.h>

#include "pages.h"

typedef struct Inode {
    int refs; // reference count
    int mode; // permission & type
    int16_t size[2]; // bytes
    int16_t ptrs[2]; // direct pointers
    int iptr; // single indirect pointer
    int inum; // store inum in itself
} Inode;

Inode* get_inode(int inum);
int inode_find(const char *path);
int alloc_inode(const char *path);
void write_inode(Inode d, int32_t pos);

#endif

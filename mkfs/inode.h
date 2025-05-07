// based on cs3650 starter code

#ifndef INODE_H
#define INODE_H

#include <stdint.h>

#include "pages.h"

typedef struct inode {
    int32_t refs; // reference count
    int32_t mode; // permission & type
    int16_t size[2]; // bytes
    int16_t ptrs[2]; // direct pointers
    int32_t iptr; // single indirect pointer
    int32_t inum; // store inum in itself
} inode;

inode* get_inode(int inum);
int inode_find(const char *path);
int alloc_inode(const char *path);


#endif

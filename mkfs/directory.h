// based on cs3650 starter code

#ifndef DIRECTORY_H
#define DIRECTORY_H

#define DIR_NAME 48

#include "pages.h"
#include "inode.h"
#include <stdbool.h>
#include <stdint.h>

typedef struct dirent {
    char name[DIR_NAME];
    int16_t inum;
    bool active;
} dirent;

int tree_lookup(const char* path, int i);

#endif


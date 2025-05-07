// based on cs3650 starter code

#ifndef DIRECTORY_H
#define DIRECTORY_H

#define DIR_NAME 48

#include "pages.h"
#include "inode.h"
#include <stdbool.h>

typedef struct dirent {
    char name[DIR_NAME];
    int  inum;
    char type;
    bool active;
    struct dirent *parent;
    struct dirent *next;
} dirent;

int tree_lookup(const char* path, int i);

#endif


#ifndef DISK_H
#define DISK_H
#include <stdlib.h>
#include <stddef.h>

char* read(size_t size, size_t offset);

void write(char *buf, size_t size, size_t offset);

#endif

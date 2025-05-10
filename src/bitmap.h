// based on cs3650 starter code

#ifndef BITMAP_H
#define BITMAP_H
#include "pages.h"
#include <stddef.h>

size_t bitmap_get(void* bm, size_t ii);
void bitmap_put(void* bm, size_t ii, size_t vv);

#endif

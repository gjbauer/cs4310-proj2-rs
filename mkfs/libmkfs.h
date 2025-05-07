#ifndef MFS_H
#define MFS_H
int
write(const char *path, const char *buf, size_t size, off_t offset);
int
mknod(const char *path, int mode);
int
mkdir(const char *path, int mode);
int
find_parent(const char* path);
#endif

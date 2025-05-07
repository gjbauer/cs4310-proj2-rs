typedef struct dirent {
    char name[DIR_NAME];
    int16_t inum;
    bool active;
} dirent;

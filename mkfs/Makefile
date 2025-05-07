
SRCS := $(wildcard *.c)
OBJS := $(SRCS:.c=.o)
HDRS := $(wildcard *.h)

CFLAGS := -g 
LDLIBS := 

mkfs: $(OBJS)
	clang $(CLFAGS) -o mkfs mkfs.o pages.o bitmap.o inode.o directory.o hash.o libmkfs.o $(LDLIBS)

%.o: %.c $(HDRS)
	clang $(CFLAGS) -c -o $@ $<

clean: unmount
	rm mkfs *.o

open:
	nvim *.c *.h

.PHONY: clean mount unmount gdb

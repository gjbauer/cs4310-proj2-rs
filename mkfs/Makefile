
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

#mount: nufs
#	mkdir -p mnt || true
#	./nufs -s -f mnt data.nufs

#unmount:
#	fusermount -u mnt || true

test: nufs
	perl test.pl

#gdb: nufs
#	mkdir -p mnt || true
#	gdb --args ./nufs -s -f mnt data.nufs

open:
	gedit *.c *.h

.PHONY: clean mount unmount gdb

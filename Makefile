
SRCS := $(wildcard *.c)
OBJS := $(SRCS:.c=.o)
HDRS := $(wildcard *.h)

CFLAGS := -g 
LDLIBS := 

nufs: $(OBJS)
	cargo run

mkfs:
	cd mkfs && make mkfs

%.o: %.c $(HDRS)
	gcc $(CFLAGS) -c -o $@ $<

clean: unmount
	rm -f nufs *.o test.log data.nufs mkfs/mkfs
	rmdir mnt || true

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

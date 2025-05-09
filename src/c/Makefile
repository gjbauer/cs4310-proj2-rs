
SRCS := $(wildcard *.c)
OBJS := $(SRCS:.c=.o)
HDRS := $(wildcard *.h)

CFLAGS := -g -c -fPIC
LDLIBS := -shared 

pages.a: $(OBJS)
	clang $(CLFAGS) -o $@ $^ $(LDLIBS)

%.o: %.c $(HDRS)
	clang $(CFLAGS) -c -o $@ $<

clean:
	rm -f pages.a *.o

.PHONY: clean


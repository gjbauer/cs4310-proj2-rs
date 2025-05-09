#include <stdint.h>
#include "pages.h"

char* read(size_t size, size_t offset) {
	char *bytes = (char*) malloc( size * sizeof(char) );
	for( int i = 0 ; i < size ; i++ ) bytes[i] = ( (char*) get_root_start() + offset + i );
	bytes[size] = '\0';
	return bytes;
}

void write(char *buf, size_t size, size_t offset) {
	for( int i = 0 ; i < size ; i++ ) ( (char*) get_root_start() + offset + i ) = buf[i] ;
}

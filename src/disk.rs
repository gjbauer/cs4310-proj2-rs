
unsafe extern "C" {
	pub fn read(size: usize, offset: usize) -> [u8];
	pub fn write(buf: [u8], size: usize, offset: usize);
}


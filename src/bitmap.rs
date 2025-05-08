
pub fn bitmap_get(mmap: &memmap2::MmapMut, ii: i32) -> u8 {
	return mmap[ii..ii+1][0];
}

pub fn bitmap_put(mmap: &memmap2::MmapMut, ii: i32, vv: u8) -> u8 {
	mmap[ii..ii+1][0] = vv;
	mmap.flush()?;
	return vv;
}


#!/bin/sh

bindgen src/c/bitmap.h > src/bitmap.rs

bindgen src/c/disk.h > src/disk.rs

bindgen src/c/pages.h > src/pages.rs


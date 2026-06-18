use memmap::MmapMut;

use crate::{HEIGHT, WIDTH};

pub fn flush(screen: &[[(u8, u8, u8); WIDTH]; HEIGHT], mmap: &mut MmapMut) {
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            mmap[j * 1280 + i * 4 + 0] = screen[j][i].2;
            mmap[j * 1280 + i * 4 + 1] = screen[j][i].1;
            mmap[j * 1280 + i * 4 + 2] = screen[j][i].0;
        }
    }
}

use memmap::MmapMut;

pub fn write(mmap: &mut MmapMut, x: usize, y: usize, rgb: (u8, u8, u8)) {
    let i: usize = (x + y * 320) * 4;
    mmap[i] = rgb.2;
    mmap[i + 1] = rgb.1;
    mmap[i + 2] = rgb.0;
}

pub fn clear(mmap: &mut MmapMut, rgb: (u8, u8, u8)) {
    for i in 0..mmap.len() {
        match i % 4 {
            0 => mmap[i] = rgb.2,
            1 => mmap[i] = rgb.1,
            2 => mmap[i] = rgb.0,
            _ => {}
        }
    }
}

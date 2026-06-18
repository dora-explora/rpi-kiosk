use memmap::MmapMut;
use embedded_graphics::{
    draw_target::DrawTarget,
    framebuffer::{Framebuffer, buffer_size},
    geometry::{OriginDimensions, Size},
    Pixel,
    pixelcolor::{Rgb888, raw::{LittleEndian, RawU24}},
};

use crate::{HEIGHT, WIDTH};

pub struct TFT {
    pub mmap: MmapMut,
    pub fb: Framebuffer::<Rgb888, RawU24, LittleEndian, 320, 240, {buffer_size::<Rgb888>(320, 240)}>,
}

impl TFT {
    pub fn new(mmap: MmapMut) -> TFT {
        TFT {
            mmap,
            fb: Framebuffer::new(),
        }
    }

    pub fn flush(&mut self) {
        let screen = self.fb.data();
        for i in 0..(WIDTH * HEIGHT) {
            self.mmap[i * 4 + 0] = screen[i * 3 + 0];
            self.mmap[i * 4 + 1] = screen[i * 3 + 1];
            self.mmap[i * 4 + 2] = screen[i * 3 + 2];
        }
    }
}

impl OriginDimensions for TFT {
    fn size(&self) -> Size {
        return Size::new(WIDTH as u32, HEIGHT as u32);
    }
}

impl DrawTarget for TFT {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>
    {
        let vec: Vec<Pixel<Rgb888>> = pixels.into_iter().collect();
        for pixel in vec { self.fb.set_pixel(pixel.0, pixel.1); }
        return Ok(());
    }
}

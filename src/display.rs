use memmap::MmapMut;
use embedded_graphics::{
    Pixel,
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{Rgb888, RgbColor}
};

use crate::{HEIGHT, WIDTH};

pub struct TFT {
    pub mmap: MmapMut,
}

impl TFT {
    pub fn new(mmap: MmapMut) -> TFT { TFT { mmap } }

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
        for pixel in vec {
            if pixel.0.x < 0 || pixel.0.x >= WIDTH as i32 || pixel.0.y < 0 || pixel.0.y >= HEIGHT as i32 { continue; }
            self.mmap[(pixel.0.y as usize * WIDTH + (pixel.0.x as usize)) * 4 + 0] = pixel.1.r();
            self.mmap[(pixel.0.y as usize * WIDTH + (pixel.0.x as usize)) * 4 + 1] = pixel.1.g();
            self.mmap[(pixel.0.y as usize * WIDTH + (pixel.0.x as usize)) * 4 + 2] = pixel.1.b();
        }
        return Ok(());
    }
}

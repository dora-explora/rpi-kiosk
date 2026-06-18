use std::{
    fs::File,
    io,
    sync::{Arc, Mutex, TryLockError::{Poisoned, WouldBlock}},
    thread::{sleep, spawn},
    time::Duration
};

use embedded_graphics::{draw_target::DrawTarget, geometry::Dimensions, pixelcolor::Rgb888, prelude::*, primitives::{Primitive, PrimitiveStyle, Rectangle}};
use linuxfb::{Framebuffer, set_terminal_mode, TerminalMode};

mod display;
mod input;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

fn main() -> io::Result<()> {
    let fbdev = Framebuffer::new("/dev/fb1").expect("could not open /dev/fb1 as framebuffer");
    let mmap = fbdev.map().expect("could not open framebuffer memmap");
    let mut tft = display::TFT::new(mmap);

    match File::open("/dev/tty1") {
        Ok(tty) => _ = set_terminal_mode(&tty, TerminalMode::Graphics),
        Err(_) => {},
    }

    let mut input = input::Input {t: false, x: 0, y: 0, p: 0};
    let inputam = Arc::new(Mutex::new(input.clone()));

    let inputamclone = inputam.clone();
    spawn(move || input::input(inputamclone));

    loop {
        match inputam.try_lock() {
            Err(Poisoned(_)) => {}, // println!("poisoned error {e} from input thread"),
            Err(WouldBlock) => {}, // println!("would block error from input thread"),
            Ok(g) => {input.t = g.t; input.x = g.x; input.y = g.y; input.p = g.p}
        }

        tft.fb.clear(Rgb888::new(22, 15, 28));
        tft.fb.bounding_box().into_styled(PrimitiveStyle::with_stroke(Rgb888::WHITE, 4)).draw(&mut tft.fb).unwrap();

        tft.flush();
        sleep(Duration::from_millis(10));
    }
    // return Ok(());
}

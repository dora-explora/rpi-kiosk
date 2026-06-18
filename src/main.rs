use std::{
    fs::File,
    io,
    sync::{Arc, Mutex, TryLockError::{Poisoned, WouldBlock}},
    thread::{sleep, spawn},
    time::Duration
};

use chrono::Local;
use embedded_graphics::{prelude::*, draw_target::DrawTarget, mono_font::{MonoTextStyle, jis_x0201::FONT_9X15}, pixelcolor::Rgb888, primitives::{Primitive, PrimitiveStyle, Rectangle}, text::Text};
use linuxfb::{Framebuffer, set_terminal_mode, TerminalMode};

mod display;
mod input;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

fn main() -> io::Result<()> {
    // set tty1 to graphics mode (if possible)
    match File::open("/dev/tty1") {
        Ok(tty) => _ = set_terminal_mode(&tty, TerminalMode::Graphics),
        Err(_) => {},
    }

    // open fb1 and grab the corresponding mmap (basically mut [u8])
    let fbdev = Framebuffer::new("/dev/fb1").expect("could not open /dev/fb1 as framebuffer");
    let mmap = fbdev.map().expect("could not open framebuffer memmap");
    let mut tft = display::TFT::new(mmap); // construct new tft from it

    // make new input data struct and arc-mutex
    let mut input = input::Input {t: false, x: 0, y: 0, p: 0};
    let inputam = Arc::new(Mutex::new(input.clone()));

    // spawn input thread with that arc-mutex
    let inputamclone = inputam.clone();
    spawn(move || input::input(inputamclone));

    // main rendering loop
    let style = MonoTextStyle::new(&FONT_9X15, Rgb888::WHITE);
    loop {
        // try to get up-to-date input information
        match inputam.try_lock() {
            Err(_) => {}, // an error just means that the other thread is using it currently
            // Err(Poisoned(e)) => println!("poisoned error {e} from input thread"),
            // Err(WouldBlock) => println!("would block error from input thread"),
            Ok(g) => {input.t = g.t; input.x = g.x; input.y = g.y; input.p = g.p}
        }

        tft.fb.clear(Rgb888::BLACK);

        let time = Local::now().time();
        Text::new(&time.format("%-I:%M:%S %p").to_string().as_str(), Point::new(4, 14), style).draw(&mut tft.fb).unwrap();
        tft.fb.bounding_box().into_styled(PrimitiveStyle::with_stroke(Rgb888::WHITE, 4)).draw(&mut tft.fb).unwrap();

        tft.flush();
        sleep(Duration::from_millis(300));
    }
    // return Ok(());
}

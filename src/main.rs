use std::{
    fs::File,
    io::Result,
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::{Duration, Instant}
};

use chrono::Local;
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{MonoTextStyle, jis_x0201::FONT_9X15},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, StyledDrawable},
    text::Text
};
use linuxfb::{Framebuffer, set_terminal_mode, TerminalMode};

mod display;
mod input;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;
const FRAMETIME: Duration = Duration::from_millis(0); // target frametime in milliseconds

fn main() -> Result<()> {
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
    let white_text_style = MonoTextStyle::new(&FONT_9X15, Rgb888::WHITE);
    let grey_text_style = MonoTextStyle::new(&FONT_9X15, Rgb888::new(0x80, 0x80, 0x80));
    let grey_stroke = PrimitiveStyle::with_stroke(Rgb888::new(0x80, 0x80, 0x80), 3);
    loop {
        // try to get up-to-date input information
        match inputam.try_lock() {
            Err(_) => {}, // an error just means that the other thread is using it currently
            Ok(g) => {input.t = g.t; input.x = g.x; input.y = g.y; input.p = g.p}
        }

        tft.clear(Rgb888::BLACK);

        let start = Instant::now();
        let now = Local::now();
        Text::new(&now.format("%-I:%M:%S %p %a %b %d").to_string().as_str(), Point::new(4, 14), white_text_style).draw(&mut tft).unwrap();
        Text::new(format!("{}, {}", input.x, input.y).as_str(), Point::new(4, 32), if input.t { white_text_style } else { grey_text_style }).draw(&mut tft).unwrap();

        if input.t {
            Circle::with_center(Point::new(input.x as i32, input.y as i32), 64 - (input.p as u32)/4).draw_styled(&grey_stroke, &mut tft);
        }

        Text::new(format!("{}us", start.elapsed().as_micros()).as_str(), Point::new(4, 50), white_text_style).draw(&mut tft).unwrap();
        if start.elapsed() < FRAMETIME { sleep(FRAMETIME - start.elapsed()); }
    }
}

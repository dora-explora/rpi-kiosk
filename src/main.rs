use std::{
    fs::File,
    io,
    sync::{Arc, Mutex, TryLockError::{Poisoned, WouldBlock}},
    thread::{sleep, spawn},
    time::Duration
};

use linuxfb::{Framebuffer, set_terminal_mode, TerminalMode};

mod display;
mod input;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

fn main() -> io::Result<()> {
    let fb = Framebuffer::new("/dev/fb1").expect("could not open /dev/fb1 as framebuffer");
    let mut mmap = fb.map().expect("could not open framebuffer memmap");
    let mut screen: [[(u8, u8, u8); WIDTH]; HEIGHT];
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

        screen = [[(0, 0, 0); WIDTH]; HEIGHT];
        let r = 50 - (input.p / 4).min(50);
        for i in (input.x - r.min(input.x))..(input.x + r) {
            for j in (input.y - r.min(input.y))..(input.y + r) {
                if input.t {
                    screen[j.clamp(0, HEIGHT - 1)][i.clamp(0, WIDTH - 1)] = (0xFF, 0xFF, 0xFF);
                } else {
                    screen[j.clamp(0, HEIGHT - 1)][i.clamp(0, WIDTH - 1)] = (0xFF, 0x00, 0xFF);
                }
            }
        }
        display::flush(&screen, &mut mmap);

        sleep(Duration::from_millis(10));
    }
    // return Ok(());
}

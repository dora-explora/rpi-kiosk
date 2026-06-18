use std::{
    io,
    sync::{Arc, Mutex, TryLockError::{Poisoned, WouldBlock}},
    thread::{sleep, spawn},
    time::Duration
};

use linuxfb::Framebuffer;

mod display;
mod input;

fn main() -> io::Result<()> {
    let fb = Framebuffer::new("/dev/fb1").expect("could not open /dev/fb1 as framebuffer");
    let mut screen = fb.map().expect("could not open framebuffer memmap");

    let mut input = input::Input {t: false, x: 0, y: 0, p: 0};
    let inputam = Arc::new(Mutex::new(input.clone()));

    let inputamclone = inputam.clone();
    spawn(move || input::input(inputamclone));

    loop {
        match inputam.try_lock() {
            Err(Poisoned(e)) => println!("poisoned error {e} from input thread"),
            Err(WouldBlock) => println!("would block error from input thread"),
            Ok(g) => {input.t = g.t; input.x = g.x; input.y = g.y; input.p = g.p}
        }

        if input.t {
            for i in 0..input.x {
                for j in 0..240 {
                    display::write(&mut screen, i as usize, j as usize, (0x00, 0x00, 0x00));
                }
            }
            for i in input.x..320 {
                for j in 0..240 {
                    display::write(&mut screen, i as usize, j as usize, (0xFF, 0xFF, 0xFF));
                }
            }
        }

        println!("{:?}", input);

        sleep(Duration::from_millis(50));
    }
    // return Ok(());
}

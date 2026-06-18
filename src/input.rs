use std::sync::{Arc, Mutex, TryLockError::{Poisoned, WouldBlock}};
use evdev::{Device, EventType};

#[derive(Clone, Copy, Debug)]
pub struct Input {
    pub t: bool,// whether or not it is currently being touched
    pub x: usize, // last known horizontal position
    pub y: usize, // last known vertical position
    pub p: usize  // last known pressure value
}

pub fn input(inputam: Arc<Mutex<Input>>) {
    let mut touch = Device::open("/dev/input/touchscreen").expect("could not open /dev/input/touchscreen as input device");
    let mut input = Input {t: false, x: 0, y: 0, p: 0};
    let mut dirty: bool = false; // true if inputam needs updating

    loop {
        for ev in touch.fetch_events().unwrap() {
            if ev.event_type() == EventType::KEY {
                input.t = false;
                continue;
            } else if ev.event_type() != EventType::ABSOLUTE {
                // println!("unexpected event: {:?}", ev.destructure());
                continue;
            }

            input.t = true;
            dirty = true;
            match ev.code() {
                0 => input.y = ((ev.value() as usize).clamp(400, 3760) - 400) / 14,
                1 => input.x = ((ev.value() as usize).clamp(400, 3600) - 400) / 10,
                24 => { let p = ev.value() as usize; if p == 0 { input.t = false } else { input.p = p }},
                _ => dirty = false
            }
        }

        if dirty {
            match inputam.try_lock() {
                Err(Poisoned(e)) => println!("poisoned error {e} from input thread"),
                Err(WouldBlock) => println!("would block error from input thread"),
                Ok(mut g) => {g.t = input.t; g.x = input.x; g.y = input.y; g.p = input.p}
            }
        }
    }
}

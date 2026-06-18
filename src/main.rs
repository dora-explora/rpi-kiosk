use std::{thread::sleep, time::Duration, fs::File, io};
use evdev::Device;
use linuxfb::Framebuffer;
use memmap::MmapMut;
use nix::sys::epoll;

fn write(mmap: &mut MmapMut, x: usize, y: usize, rgb: (u8, u8, u8)) {
    let i: usize = (x + y * 320) * 4;
    mmap[i] = rgb.2;
    mmap[i + 1] = rgb.1;
    mmap[i + 2] = rgb.0;
}

fn clear(mmap: &mut MmapMut, rgb: (u8, u8, u8)) {
    for i in 0..mmap.len() {
        match i % 4 {
            0 => mmap[i] = rgb.2,
            1 => mmap[i] = rgb.1,
            2 => mmap[i] = rgb.0,
            _ => {}
        }
    }
}

fn main() -> io::Result<()> {
    let fb = Framebuffer::new("/dev/fb1").expect("could not open /dev/fb1 as framebuffer");
    let mut screen = fb.map().expect("could not open framebuffer memmap");

    let mut touch = Device::open("/dev/input/touchscreen")?;
    // thank you so much evdev docs
    touch.set_nonblocking(true)?;
    let epoll = epoll::Epoll::new(epoll::EpollCreateFlags::EPOLL_CLOEXEC)?;
    let event = epoll::EpollEvent::new(epoll::EpollFlags::EPOLLIN, 0);
    epoll.add(&touch, event)?;
    let mut events = [epoll::EpollEvent::empty(); 2];

    loop {
        match touch.fetch_events() {
            Ok(iterator) => {
                for ev in iterator {
                    println!("{ev:?}");

                }
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Wait forever for bytes available on dev
                epoll.wait(&mut events, epoll::EpollTimeout::NONE)?;
            }
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        }
        println!("---------------------------------------------------")
    }
    return Ok(());
}

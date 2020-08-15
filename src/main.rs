use std::thread;
use std::time::{Duration, Instant};
use crossterm::terminal;

use doomfire::{DoomFire, FIRE_HEIGHT, FIRE_WIDTH, TIME_PER_FRAME};

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn main() {
    let mut doom_fire = DoomFire::new();

    let mut frame: Vec<u8> = vec![0; FIRE_WIDTH * FIRE_HEIGHT * 4];
    let mut buffer: Vec<u32> = vec![0; FIRE_WIDTH * FIRE_HEIGHT];

    let mut term_width = 0 as u32;
    let mut term_height = 0 as u32;

    match terminal::size() {
        Ok(res) => {
            term_width = res.0 as u32 * 8;
            term_height = (res.1 - 1) as u32 * 8 * 2;
        }
        Err(_) => {}
    }

    let mut engine = blockish::ThreadedEngine::new(term_width, term_height, false);

    loop {
        let start_time = Instant::now();
        println!("\x1b[{};0f", 0);
        doom_fire.draw(&mut frame);

        // DoomFire expects a &[u8] to write the pixels with a RGBA encoding but minifb
        // expects a &[u32] with a 0RGB pixel encoding, where the upper 8 bits are ignored.
        for (i, pixel) in frame.chunks_exact(4).enumerate() {
            buffer[i] = from_u8_rgb(pixel[0], pixel[1], pixel[2]);
        }

        engine.render(&|x, y| {
            let start = (y * FIRE_HEIGHT as u32 / term_height * FIRE_WIDTH as u32 + (x * FIRE_WIDTH as u32 / term_width))
                as usize;
            let pixel = buffer[start];
            (
                (pixel >> 16 & 0xff) as u8,
                (pixel >> 8 & 0xff) as u8,
                (pixel & 0xff) as u8,
            )
        });
        doom_fire.update();
        let end_time = Instant::now();
        let render_time = end_time - start_time;
        if render_time < Duration::from_millis(TIME_PER_FRAME) {
            let waste_time = Duration::from_millis(TIME_PER_FRAME) - render_time;
            thread::sleep(waste_time);
        }
    }
}

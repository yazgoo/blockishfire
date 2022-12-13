extern crate rand;

use rand::Rng;

use crossterm::terminal;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    const COLOR_PALLET: [[u8; 3]; 37] = [
        [0x07, 0x07, 0x07],
        [0x1F, 0x07, 0x07],
        [0x2F, 0x0F, 0x07],
        [0x47, 0x0F, 0x07],
        [0x57, 0x17, 0x07],
        [0x67, 0x1F, 0x07],
        [0x77, 0x1F, 0x07],
        [0x8F, 0x27, 0x07],
        [0x9F, 0x2F, 0x07],
        [0xAF, 0x3F, 0x07],
        [0xBF, 0x47, 0x07],
        [0xC7, 0x47, 0x07],
        [0xDF, 0x4F, 0x07],
        [0xDF, 0x57, 0x07],
        [0xDF, 0x57, 0x07],
        [0xD7, 0x5F, 0x07],
        [0xD7, 0x5F, 0x07],
        [0xD7, 0x67, 0x0F],
        [0xCF, 0x6F, 0x0F],
        [0xCF, 0x77, 0x0F],
        [0xCF, 0x7F, 0x0F],
        [0xCF, 0x87, 0x17],
        [0xC7, 0x87, 0x17],
        [0xC7, 0x8F, 0x17],
        [0xC7, 0x97, 0x1F],
        [0xBF, 0x9F, 0x1F],
        [0xBF, 0x9F, 0x1F],
        [0xBF, 0xA7, 0x27],
        [0xBF, 0xA7, 0x27],
        [0xBF, 0xAF, 0x2F],
        [0xB7, 0xAF, 0x2F],
        [0xB7, 0xB7, 0x2F],
        [0xB7, 0xB7, 0x37],
        [0xCF, 0xCF, 0x6F],
        [0xDF, 0xDF, 0x9F],
        [0xEF, 0xEF, 0xC7],
        [0xFF, 0xFF, 0xFF],
    ];
    let fire_width = 320;
    let fire_height = 168;
    let time_per_frame = 1000 / 60;
    let mut doom_fire = vec![0; fire_width * fire_height];

    let mut term_width = 0 as u32;
    let mut term_height = 0 as u32;

    match terminal::size() {
        Ok(res) => {
            term_width = res.0 as u32 * 8;
            term_height = res.1 as u32 * 8 * 2;
        }
        Err(_) => {}
    }

    let mut engine = blockish::ThreadedEngine::new(term_width, term_height, false);
    for i in 0..fire_width {
        doom_fire[(fire_height - 1) * fire_width + i] = 36;
    }

    let mut rng = rand::thread_rng();
    loop {
        let start_time = Instant::now();
        print!("\x1b[{};0f", 0);
        for x in 0..fire_width {
            for y in 1..fire_height {
                let src = y * fire_width + x;
                if doom_fire[src] > 0 {
                    let r = (rng.gen_range(0.0, 3.0) + 0.5) as usize & 3;
                    let dst = (src - r + 1) as usize;
                    let res = doom_fire[src] - (r & 1);
                    doom_fire[dst - fire_width] = res;
                } else {
                    doom_fire[src - fire_width] = 0;
                }
            }
        }

        engine.render(&|x, y| {
            let start = (y * fire_height as u32 / term_height * fire_width as u32
                + (x * fire_width as u32 / term_width)) as usize;
            let pixel = doom_fire[start];
            let rgb = COLOR_PALLET[pixel];
            (rgb[0], rgb[1], rgb[2], if rgb[0] == 0x7 { 0 } else { 254 })
        });
        let end_time = Instant::now();
        let render_time = end_time - start_time;
        if render_time < Duration::from_millis(time_per_frame) {
            let waste_time = Duration::from_millis(time_per_frame) - render_time;
            thread::sleep(waste_time);
        }
    }
}

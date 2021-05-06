use console_engine::{Color, ConsoleEngine};
use crate::stack::Stack;
use console_engine::pixel;
use console_engine::screen::Screen;
use std::sync::MutexGuard;

pub struct Display {
}

impl Display {
    pub fn new() -> Self {
        Display {
        }
    }

    pub fn draw_sprite(&mut self, engine: &mut ConsoleEngine, orig_x: u8, orig_y: u8, rows: u16, stack: &mut MutexGuard<Stack>) {
        let mut mem = stack.i;
        let mut x = orig_x;
        let mut y = orig_y;
        for n in 0..(rows as i32) {
            if y > 32 {
                break;
            }
            let sprite_data = stack.memory[mem as usize];
            mem = mem + 1u16;
            for pixel in (0..8).rev() {
                if x > 64 {
                    break;
                }

                let pixel_data = Display::get_bit_at(sprite_data, pixel);
                let mut current_pixel = engine.get_pxl(x as i32, y as i32).unwrap();
                if pixel_data && current_pixel.chr == '█' {
                    engine.set_pxl(x as i32, y as i32, pixel::pxl_fg(' ', Color::White));
                    stack.registers.set_register(0xf, 1);
                } else if pixel_data && current_pixel.chr != '█' {
                    engine.set_pxl(x as i32, y as i32, pixel::pxl_fg('█', Color::White));
                }
                x += 1;
            }
            x = orig_x;
            y += 1;
        }
    }

    fn get_bit_at(input: u8, n: u8) -> bool {
        if n < 8 {
            input & (1 << n) != 0
        } else {
            false
        }
    }
}
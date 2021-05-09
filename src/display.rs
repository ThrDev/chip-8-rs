use crate::stack::Stack;
use std::sync::{MutexGuard, Mutex, Arc, RwLock};
use minifb::{Window, WindowOptions, Key};

pub struct Display {
}

impl Display {
    pub fn new() -> Self {
        Display {
        }
    }

    pub fn clear(buffer: &mut Vec<u32>) {
        buffer.fill(0);
    }

    pub fn refresh(window: &Window) -> bool {
        window.is_open() && !window.is_key_down(Key::Escape)
    }

    pub fn draw_sprite(buffer: &mut Arc<RwLock<Vec<u32>>>,
                       orig_x: u8,
                       orig_y: u8,
                       rows: u16,
                       stack: &mut MutexGuard<Stack>) -> Vec<u32> {
        let mut mem = stack.i;
        let mut x = orig_x as u32;
        let mut y = orig_y as u32;
        let mut buff: Vec<u32>;
        {
            let gotten_buffer = buffer.read().unwrap();
            buff = gotten_buffer.clone();
        }
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
                let mut index = x + (y * 64);
                let cloned_buf = buff.clone();
                let current_char = cloned_buf.get(index as usize);
                if current_char == Some(&255u32) && pixel_data {
                    std::mem::replace(&mut buff[index as usize], 0);
                    stack.registers.set_register(0xf, 1);
                }
                if current_char == Some(&0u32) && pixel_data {
                    std::mem::replace(&mut buff[index as usize], 255);
                }
                x += 1;
            }
            x = orig_x as u32;
            y += 1;
        }

        buff
    }

    fn get_bit_at(input: u8, n: u8) -> bool {
        if n < 8 {
            input & (1 << n) != 0
        } else {
            false
        }
    }
}
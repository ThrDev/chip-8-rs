use crate::stack::Stack;
use std::sync::MutexGuard;
use glerminal::{TerminalBuilder, TextBuffer, Terminal, TermCharacter, VirtualKeyCode, Events};

pub struct Display {
    terminal: Terminal,
    text_buffer: TextBuffer,
    events: Option<Events>,
}

impl Display {
    pub fn new() -> Self {
        let terminal = TerminalBuilder::new()
            .with_title("CHIP-8 EMU")
            .with_dimensions((1280,720))
            .build();

        let mut text_buffer = Display::new_buffer(&terminal);
        terminal.flush(&mut text_buffer);

        Display {
            terminal,
            text_buffer,
            events: None
        }
    }

    pub fn clear(&mut self) {
        let mut text_buffer = Display::new_buffer(&self.terminal);
        self.terminal.flush(&mut text_buffer);
        self.text_buffer = text_buffer;
    }

    pub fn draw(&mut self) {
        self.terminal.draw(&mut self.text_buffer);
    }

    pub fn refresh(&mut self) -> bool {
        let refreshed = self.terminal.refresh();
        let events = self.terminal.get_current_events();
        self.events = Some(events);
        return refreshed;
    }

    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        let keys_pressed = &self.events.as_ref().unwrap().keyboard.get_pressed_list();
        keys_pressed.contains(&key)
    }

    pub fn draw_sprite(&mut self, orig_x: u8, orig_y: u8, rows: u16, stack: &mut MutexGuard<Stack>) {
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

                let mut current_char = self.text_buffer.get_character(x as u32, y as u32);
                match current_char {
                    Some(c) => {
                        let chr = c.get_char();
                        if chr == '█' && pixel_data {
                            self.text_buffer.cursor.move_to(x as u32, y as u32);
                            self.text_buffer.write(' ');
                            self.terminal.flush(&mut self.text_buffer);
                            stack.registers.set_register(0xf, 1);
                        }
                        if chr == ' ' as char && pixel_data {
                            //self.text_buffer.set_char(x as u32, y as u32, TermCharacter{ character: 0, style: Default::default() });
                            self.text_buffer.cursor.move_to(x as u32, y as u32);
                            self.text_buffer.write('█');
                            self.terminal.flush(&mut self.text_buffer);
                        }
                    },
                    None => ()
                }
                x += 1;
            }
            x = orig_x;
            y += 1;
        }
    }

    fn new_buffer(terminal: &Terminal) -> TextBuffer {
        let mut text_buffer;
        match TextBuffer::create(&terminal, (64, 32)) {
            Ok(buffer) => text_buffer = buffer,
            Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
        }
        return text_buffer
    }

    fn get_bit_at(input: u8, n: u8) -> bool {
        if n < 8 {
            input & (1 << n) != 0
        } else {
            false
        }
    }
}
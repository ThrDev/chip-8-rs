#![feature(allocator_api)]
// CHIP-8 EMU

use crate::vm::Vm;
use std::env;
use minifb::{Window, WindowOptions};

mod vm;
mod instruction;
mod stack;
mod opcode;
mod register;
mod display;
mod keymap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("Please pass a filename to the command line.");
    }

    let mut window = Window::new(
        "Test - ESC to exit",
        640,
        320,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16400)));

    let mut vm = Vm::new();
    vm.start();

    match std::fs::read(&args[1]) {
        Ok(bytes) => { vm.load(bytes); }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("File not found!");
                return;
            }
            panic!("{}", e);
        }
    }

    println!("Executing...");
    vm.update();
    vm.execute(&mut window);
    vm.stop();
}

// CHIP-8 EMU

use crate::vm::Vm;
use std::sync::Arc;
use std::env;

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
    vm.execute();
    vm.stop();

    println!("Hello world, from RPI-Zero-W!");
}

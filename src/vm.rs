use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex, MutexGuard, RwLock};
use std::sync::mpsc::{TryRecvError, Sender, Receiver};
use crate::instruction::Instruction;
use std::thread::JoinHandle;
use crate::stack::{Stack};
use crate::opcode::OpCode;
use crate::register::Register;
use rand::Rng;
use crate::display::Display;
use crate::keymap::KeyMap;
use std::process::exit;
use minifb::{Key, Window, KeyRepeat};
use std::alloc::Global;

pub struct Vm {
    pub delay_timer: Arc<Mutex<Timer>>,
    pub sound_timer: Arc<Mutex<Timer>>,
    pub stack: Arc<Mutex<Stack>>,
    pub keys_down: Arc<RwLock<KeysDown>>,
    pub buffer: Arc<RwLock<Vec<u32>>>,
    pub needs_update: Arc<RwLock<Update>>,
    pub display: Display,
    pub execution_thread: Option<JoinHandle<()>>
}

pub struct Timer {
    pub timing: u8,
    pub tx: Sender<i32>,
    pub rx: Receiver<i32>
}

pub struct KeysDown {
    pub keys_down: Option<Vec<Key, Global>>
}

pub struct Update {
    pub needs: bool
}

impl Vm {
    pub fn new() -> Self {
        let (delaytx, delayrx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let (soundtx, soundrx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

        let mut vm = Vm{
            needs_update: Arc::new(RwLock::new(Update{needs:true})),
            keys_down: Arc::new(RwLock::new(KeysDown{keys_down:None})),
            buffer: Arc::new(RwLock::new(vec![0; 64 * 32])),
            display: Display::new(),
            delay_timer: Arc::new(Mutex::new(Timer {
                timing: 0,
                tx: delaytx,
                rx: delayrx
            })),
            sound_timer: Arc::new(Mutex::new(Timer {
                timing: 0,
                tx: soundtx,
                rx: soundrx
            })),
            stack: Arc::new(Mutex::new(Stack {
                i: 0,
                counter: 0,
                registers: Register {
                    v0: 0,
                    v1: 0,
                    v2: 0,
                    v3: 0,
                    v4: 0,
                    v5: 0,
                    v6: 0,
                    v7: 0,
                    v8: 0,
                    v9: 0,
                    va: 0,
                    vb: 0,
                    vc: 0,
                    vd: 0,
                    ve: 0,
                    vf: 0
                },
                call_stack: vec![],
                memory: vec![0; 4096]
            })),
            execution_thread: None
        };
        vm.load_font();
        vm
    }

    pub fn start(&mut self) {
        let delay_timer_clone = Arc::clone(&self.delay_timer);
        thread::spawn(move || loop {
            {
                let mut delay_timer = delay_timer_clone.lock().unwrap();
                if delay_timer.timing > 0 {
                    delay_timer.timing -= 1;
                }

                match delay_timer.rx.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        break;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }
            thread::sleep(Duration::from_millis(1000 / 60));
        });

        let sound_timer_clone = Arc::clone(&self.delay_timer);
        thread::spawn(move || loop {
            {
                let mut sound_timer = sound_timer_clone.lock().unwrap();
                if sound_timer.timing > 0 {
                    //beep(440);
                    sound_timer.timing -= 1;
                }
                else {
                    //beep(0);
                }

                match sound_timer.rx.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        println!("Stopping thread");
                        break;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }
            thread::sleep(Duration::from_millis(1000 / 60));
        });
    }

    pub fn load(&mut self, program: Vec<u8>) {
        // load the program into memory at location ?
        let mut stack = self.stack.lock().unwrap();
        let mut start_offset = 512;
        for i in program {
            stack.memory.insert(start_offset, i);
            start_offset += 1;
        }
        stack.counter = 512;
    }

    fn load_font(&mut self) {
        let font = vec!(
                        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0 0x00
                        0x20, 0x60, 0x20, 0x20, 0x70, // 1 0x05
                        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2 0x0A
                        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3 0x0F
                        0x90, 0x90, 0xF0, 0x10, 0x10, // 4 0x14
                        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5 0x19
                        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6 0x1E
                        0xF0, 0x10, 0x20, 0x40, 0x40, // 7 0x23
                        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8 0x28
                        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9 0x2D
                        0xF0, 0x90, 0xF0, 0x90, 0x90, // A 0x32
                        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B 0x37
                        0xF0, 0x80, 0x80, 0x80, 0xF0, // C 0x3C
                        0xE0, 0x90, 0x90, 0x90, 0xE0, // D 0x41
                        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E 0x46
                        0xF0, 0x80, 0xF0, 0x80, 0x80, // F 0x4B
                        );
        let mut stack = self.stack.lock().unwrap();
        let mut start_offset = 0;
        for value in font {
            stack.memory.insert(start_offset, value);
            start_offset += 1;
        }
    }

    pub fn update(&mut self) {
        let clone = Arc::clone(&self.stack);
        let mut key_clone = Arc::clone(&self.keys_down);
        let mut cloned_buffer = Arc::clone(&self.buffer);
        let mut cloned_delay_timer = Arc::clone(&self.delay_timer);

        thread::spawn(move || loop {
            {
                let mut stack = clone.lock().unwrap();
                let instruction = &stack.get_next_instruction();
                let updated_buffer = Vm::handle_instruction(&mut cloned_buffer,
                                                            &mut key_clone,
                                                            &mut stack,
                                                            &mut cloned_delay_timer,
                                                            &instruction.as_ref().unwrap());

                match updated_buffer {
                    Some(b) => {
                        let mut write_buffer = cloned_buffer.write().unwrap();
                        write_buffer.clear();
                        write_buffer.extend(b);
                    },
                    None => {
                    }
                };

                if Self::is_key_pressed(&key_clone, Key::B) {
                    exit(0); // exits app
                }
            }
        });
    }

    pub fn execute(&mut self, window: &mut Window) {
        let cloned = Arc::clone(&self.keys_down);
        let cloned_buffer = Arc::clone(&self.buffer);
        while Display::refresh(&window) {
            let pressed =  window.get_keys_pressed(KeyRepeat::Yes);
            let released = window.get_keys_released();
            let updated_keys: Option<Vec<Key>>;
            {
                let keys_down = cloned.read().unwrap();
                let kd = keys_down.keys_down.clone();
                if kd.is_some() {
                    let mut keys_down_cloned = kd.unwrap();
                    for key in pressed.unwrap().iter() {
                        if !keys_down_cloned.contains(key) {
                            keys_down_cloned.insert(keys_down_cloned.len(), *key);
                        }
                    }
                    for key in released.unwrap().iter() {
                        let item = keys_down_cloned.iter().position(|p| p == key);
                        match item {
                            Some(it) => {
                                keys_down_cloned.remove(it);
                            },
                            _ => ()
                        }
                    }
                    updated_keys = Some(keys_down_cloned);
                }
                else {
                    updated_keys = pressed;
                }
            }
            {
                let mut write_lock = cloned.write().unwrap();
                write_lock.keys_down = updated_keys;
            }

            {
                let buffer = cloned_buffer.read().unwrap();
                window
                    .update_with_buffer(&buffer, 64, 32)
                    .unwrap();
            }
        }
    }

    pub fn stop(&mut self) {
        let _ = self.delay_timer.lock().unwrap().tx.send(1);
        let _ = self.sound_timer.lock().unwrap().tx.send(1);
    }

    fn handle_instruction(buffer: &mut Arc<RwLock<Vec<u32>>>,
                          keys_down: &mut Arc<RwLock<KeysDown>>,
                          stack: &mut MutexGuard<Stack>,
                          delay_timer: &mut Arc<Mutex<Timer>>,
                          i: &Instruction) -> Option<Vec<u32>> {
        match i.opcode {
            OpCode::CLS => {
                return Some(vec![0; 64 * 32]);
            },
            OpCode::JMP => {
                stack.counter = i.bit & ((1u16 << 12) - 1);
            },
            OpCode::CALL => {
                let computed_addr = i.bit & ((1u16 << 12) - 1);
                let old_counter = stack.counter.clone();
                stack.call_stack.push(old_counter);
                stack.counter = computed_addr;
            },
            OpCode::RET => {
                let old_addr = stack.call_stack.pop();
                match old_addr {
                    Some(addr) => stack.counter = addr,
                    None => println!("no item on the stack, w0t?")
                }
            },
            OpCode::H3XNN => {
                let register_value = stack.registers.get_register(i.second);
                if register_value == (i.bit & ((1u16 << 8) - 1)) as u8 {
                    stack.counter += 2;
                }
            },
            OpCode::H4XNN => {
                let register_value = stack.registers.get_register(i.second);
                if register_value != (i.bit & ((1u16 << 8) - 1)) as u8 {
                    stack.counter += 2;
                }
            },
            OpCode::H5XY0 => {
                let first_register_value = stack.registers.get_register(i.second);
                let second_register_value = stack.registers.get_register(i.third);
                if first_register_value == second_register_value {
                    stack.counter += 2;
                }
            },
            OpCode::H9XY0 => {
                let first_register_value = stack.registers.get_register(i.second);
                let second_register_value = stack.registers.get_register(i.third);
                if first_register_value != second_register_value {
                    stack.counter += 2;
                }
            },
            OpCode::H6XNN => {
                stack.registers.set_register(i.second, (i.bit & ((1u16 << 8) - 1)) as u8);
            },
            OpCode::H7XNN => {
                let first_register_value = stack.registers.get_register(i.second);
                let converted: u16 = first_register_value as u16;
                let total_value = (i.bit & ((1u16 << 8) - 1)) + converted;
                stack.registers.set_register(i.second, total_value as u8);
            },
            OpCode::H8XY0 => {
                let value = stack.registers.get_register(i.third);
                stack.registers.set_register(i.second, value);
            },
            OpCode::H8XY1 => {
                let vx = stack.registers.get_register(i.second);
                let vy = stack.registers.get_register(i.third);
                stack.registers.set_register(i.second, vx | vy);
            },
            OpCode::H8XY2 => {
                let vx = stack.registers.get_register(i.second);
                let vy = stack.registers.get_register(i.third);
                stack.registers.set_register(i.second, vx & vy);
            },
            OpCode::H8XY3 => {
                let vx = stack.registers.get_register(i.second);
                let vy = stack.registers.get_register(i.third);
                stack.registers.set_register(i.second, vx ^ vy);
            },
            OpCode::H8XY4 => {
                let vx = stack.registers.get_register(i.second);
                let vy = stack.registers.get_register(i.third);
                let converted: u16 = vx as u16;
                let output = converted + vy as u16;
                match output {
                    _ if output > 0x255 => stack.registers.set_register(0xf, 1),
                    _ => stack.registers.set_register(0xf, 0)
                }
                stack.registers.set_register(i.second, output as u8);
            },
            OpCode::H8XY5 => {
                let vx = stack.registers.get_register(i.second);
                let vy = stack.registers.get_register(i.third);
                let converted: i16 = vx as i16;
                let output = converted - vy as i16;
                match output {
                    _ if output <= 0x0 => stack.registers.set_register(0xf, 0),
                    _ => stack.registers.set_register(0xf, 1)
                }
                stack.registers.set_register(i.second, output as u8);
            },
            OpCode::H8XY6 => {
                let vy = stack.registers.get_register(i.third);
                let shifted_right = vy >> 1;
                match shifted_right {
                    0x1 => stack.registers.set_register(0xf, 1),
                    0x0 => stack.registers.set_register(0xf, 0),
                    _ => ()
                }
                stack.registers.set_register(i.second, shifted_right);
            },
            OpCode::H8XY7 => {
                let vx = stack.registers.get_register(i.second);
                let vy = stack.registers.get_register(i.third);
                if vy < vx {
                    stack.registers.set_register(0xf, 1);
                    stack.registers.set_register(i.second, 0);
                } else {
                    stack.registers.set_register(0xf, 0);
                    stack.registers.set_register(i.second, vy - vx);
                }
            },
            OpCode::H8XYE => {
                let vy = stack.registers.get_register(i.third);
                let shifted_left = vy << 1;
                match shifted_left {
                    0x1 => stack.registers.set_register(0xf, 1),
                    0x0 => stack.registers.set_register(0xf, 0),
                    _ => ()
                }
                stack.registers.set_register(i.second, shifted_left);
            },
            OpCode::HANNN => {
                stack.i = i.bit & ((1u16 << 12) - 1);
            },
            OpCode::HBNNN => {
                let v0 = stack.registers.get_register(0x0);
                stack.counter = (i.bit & ((1u16 << 12) - 1)) + v0 as u16;
            },
            OpCode::HCXNN => {
                let mut rng = rand::thread_rng();
                let n2: u8 = rng.gen();
                stack.registers.set_register(i.second, (i.bit & ((1u16 << 8) - 1)) as u8 & n2);
            },
            OpCode::HDXYN => {
                let orig_x = stack.registers.get_register(i.second) % 64;
                let orig_y = stack.registers.get_register(i.third) % 32;
                stack.registers.set_register(0xf, 0);
                let updated_buf = Display::draw_sprite(buffer, orig_x, orig_y, i.fourth, stack);
                return Some(updated_buf);
            },
            OpCode::HEX9E => {
                // skip if key pressed
                let vx = stack.registers.get_register(i.second);
                let key = KeyMap::match_to_key(vx);
                //println!("Checking for keypress {:?}", key);
                if Vm::is_key_pressed(keys_down,key) {
                    //println!("Found keypress! {:?}", key);
                    stack.counter += 2;
                }
            },
            OpCode::HEXA1 => {
                // skip if key not pressed
                let vx = stack.registers.get_register(i.second);
                let key = KeyMap::match_to_key(vx);
                if !Vm::is_key_pressed(keys_down,key) {
                    stack.counter += 2;
                }
            },
            OpCode::HFX07 => {
                // acquire timer
                let unlocked_timer = delay_timer.lock().unwrap();
                stack.registers.set_register(i.second, unlocked_timer.timing);
            },
            OpCode::HFX15 => {
                let unlocked_timer = delay_timer.lock().unwrap();
                stack.registers.set_register(i.second, unlocked_timer.timing);
            },
            OpCode::HFX18 => {
                let vx = stack.registers.get_register(i.second);
                let cloned_timer = Arc::clone(&delay_timer);
                let mut unlocked_timer = cloned_timer.lock().unwrap();
                unlocked_timer.timing = vx;
            },
            OpCode::HFX1E => {
                let vx = stack.registers.get_register(i.second);
                stack.i += vx as u16;
                if stack.i > 0x0FFF {
                    stack.registers.set_register(0xf, 1);
                }
            },
            OpCode::HFX0A => {
                let keys = KeyMap::get_all_keys();
                loop {
                    for key in keys.iter() {
                        if Vm::is_key_pressed(keys_down,*key) {
                            let stored_key = KeyMap::match_to_u8(*key);
                            println!("Key pressed! {:?}, mapped to: {:?}", key, stored_key);
                            stack.registers.set_register(i.second, stored_key);
                            return None;
                        }
                    }
                }
            },
            OpCode::HFX29 => {
                let vx = stack.registers.get_register(i.second);
                let font = vx * 0x5;
                stack.i = font as u16;
            },
            OpCode::HFX33 => {
                let vx = stack.registers.get_register(i.second);
                let first_number = vx / 100;
                let second_number = vx / 10 % 10;
                let third_number = vx % 10;
                let l = stack.i;
                stack.memory[l as usize] = first_number;
                stack.memory[(l +1) as usize] = second_number;
                stack.memory[(l +2) as usize] = third_number;
            },
            OpCode::HFX55 => {
                let mut l = stack.i;
                let mut count: u8 = 0;
                loop {
                    let v = stack.registers.get_register(count as u16);
                    stack.memory[l as usize] = v;
                    count += 1;
                    l += 1;
                    if i.second < count as u16 {
                        break;
                    }
                }
            },
            OpCode::HFX65 => {
                let mut l = stack.i;
                let mut count: u8 = 0;
                loop {
                    let v = stack.memory[l as usize];
                    stack.registers.set_register(count as u16, v);
                    count += 1;
                    l += 1;
                    if i.second < count as u16 {
                        break;
                    }
                }
            }
            _ => {
                return None;
            }
        };

        None
    }

    fn is_key_pressed(keys_down: &Arc<RwLock<KeysDown>>, key: Key) -> bool {
        let keys_down = keys_down.read().unwrap();
        match &keys_down.keys_down {
            Some(ks) => ks.contains(&key),
            None => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_register_opcode() {
        let mut vm = make_vm(vec!(0x6001));
        let mut stack = vm.stack.lock().unwrap();
        assert_eq!(stack.registers.v0, 1);
        assert_eq!(stack.registers.get_register(0x0), 1);
    }

    #[test]
    fn test_set_register_opcode() {
        let mut vm = make_vm(vec!(0x0000));
        let mut stack = vm.stack.lock().unwrap();
        stack.registers.set_register(0x1, 0x23);
        assert_eq!(stack.registers.v1, 0x23);
        assert_eq!(stack.registers.get_register(0x1), 0x23);
    }

    #[test]
    fn test_font_opcode() {
        // load value 0x05 into v1, load font into i at value of addr v1
        let mut vm = make_vm(vec!(0x6105, 0xf129));
        assert_eq!(vm.stack.lock().unwrap().i, 0x19);
    }

    fn make_vm(ops: Vec<u16>) -> Vm {
        let mut vm = Vm::new();

        let (delaytx, delayrx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let mut cloned = Arc::clone(&vm.stack);
        let mut stack = cloned.lock().unwrap();
        let mut keys_down: Arc<RwLock<KeysDown>> = Arc::new(RwLock::new(KeysDown{keys_down:None}));
        let mut delay_timer: Arc<Mutex<Timer>> = Arc::new(Mutex::new(Timer{timing:0, tx: delaytx, rx: delayrx}));
        let mut buffer: Arc<RwLock<Vec<u32>>> = Arc::new(RwLock::new(vec![]));
        for instruction in ops {
            Vm::handle_instruction(&mut buffer, &mut keys_down, &mut stack, &mut delay_timer, &Instruction::new(instruction));
        }

        vm
    }
}
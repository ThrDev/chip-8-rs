use glerminal::VirtualKeyCode;

pub struct KeyMap {
    k_0: VirtualKeyCode,
    k_1: VirtualKeyCode,
    k_2: VirtualKeyCode,
    k_3: VirtualKeyCode,
    k_4: VirtualKeyCode,
    k_5: VirtualKeyCode,
    k_6: VirtualKeyCode,
    k_7: VirtualKeyCode,
    k_8: VirtualKeyCode,
    k_9: VirtualKeyCode,
    k_a: VirtualKeyCode,
    k_b: VirtualKeyCode,
    k_c: VirtualKeyCode,
    k_d: VirtualKeyCode,
    k_e: VirtualKeyCode,
    k_f: VirtualKeyCode,
}

impl KeyMap {
    pub fn new() -> Self {
        KeyMap {
            k_0: VirtualKeyCode::X,
            k_1: VirtualKeyCode::Key1,
            k_2: VirtualKeyCode::Key2,
            k_3: VirtualKeyCode::Key3,
            k_4: VirtualKeyCode::Q,
            k_5: VirtualKeyCode::W,
            k_6: VirtualKeyCode::E,
            k_7: VirtualKeyCode::A,
            k_8: VirtualKeyCode::S,
            k_9: VirtualKeyCode::D,
            k_a: VirtualKeyCode::Z,
            k_b: VirtualKeyCode::C,
            k_c: VirtualKeyCode::Key4,
            k_d: VirtualKeyCode::R,
            k_e: VirtualKeyCode::F,
            k_f: VirtualKeyCode::V,
        }
    }

    pub fn match_to_key(&self, key: u8) -> VirtualKeyCode {
        match key {
            0x0 => self.k_0,
            0x1 => self.k_1,
            0x2 => self.k_2,
            0x3 => self.k_3,
            0x4 => self.k_4,
            0x5 => self.k_5,
            0x6 => self.k_6,
            0x7 => self.k_7,
            0x8 => self.k_8,
            0x9 => self.k_9,
            0xA => self.k_a,
            0xB => self.k_b,
            0xC => self.k_c,
            0xD => self.k_d,
            0xE => self.k_e,
            0xF => self.k_f,
            _ => VirtualKeyCode::Unlabeled
        }
    }

    pub fn match_to_u8(&self, key: VirtualKeyCode) -> u8 {
        match key {
            x if self.k_0 == key => 0x0,
            x if self.k_1 == key => 0x1,
            x if self.k_2 == key => 0x2,
            x if self.k_3 == key => 0x3,
            x if self.k_4 == key => 0x4,
            x if self.k_5 == key => 0x5,
            x if self.k_6 == key => 0x6,
            x if self.k_7 == key => 0x7,
            x if self.k_8 == key => 0x8,
            x if self.k_9 == key => 0x9,
            x if self.k_a == key => 0xA,
            x if self.k_b == key => 0xB,
            x if self.k_c == key => 0xC,
            x if self.k_d == key => 0xD,
            x if self.k_e == key => 0xE,
            x if self.k_f == key => 0xF,
            _ => 0xFF
        }
    }

    pub fn get_all_keys(&self) -> Vec<VirtualKeyCode> {
        return vec!(self.k_0,
                    self.k_1,
                    self.k_2,
                    self.k_3,
                    self.k_4,
                    self.k_5,
                    self.k_6,
                    self.k_7,
                    self.k_8,
                    self.k_9,
                    self.k_a,
                    self.k_b,
                    self.k_c,
                    self.k_d,
                    self.k_e,
                    self.k_f);
    }
}
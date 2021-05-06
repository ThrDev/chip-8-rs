use console_engine::KeyCode;

pub struct KeyMap {
    k_0: KeyCode,
    k_1: KeyCode,
    k_2: KeyCode,
    k_3: KeyCode,
    k_4: KeyCode,
    k_5: KeyCode,
    k_6: KeyCode,
    k_7: KeyCode,
    k_8: KeyCode,
    k_9: KeyCode,
    k_a: KeyCode,
    k_b: KeyCode,
    k_c: KeyCode,
    k_d: KeyCode,
    k_e: KeyCode,
    k_f: KeyCode,
}

impl KeyMap {
    pub fn new() -> Self {
        KeyMap {
            k_0: KeyCode::Char('1'),
            k_1: KeyCode::Char('2'),
            k_2: KeyCode::Char('3'),
            k_3: KeyCode::Char('q'),
            k_4: KeyCode::Char('w'),
            k_5: KeyCode::Char('e'),
            k_6: KeyCode::Char('a'),
            k_7: KeyCode::Char('s'),
            k_8: KeyCode::Char('d'),
            k_9: KeyCode::Char('z'),
            k_a: KeyCode::Char('x'),
            k_b: KeyCode::Char('c'),
            k_c: KeyCode::Char('4'),
            k_d: KeyCode::Char('r'),
            k_e: KeyCode::Char('f'),
            k_f: KeyCode::Char('v'),
        }
    }

    pub fn match_to_key(&self, key: u8) -> KeyCode {
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
            _ => KeyCode::Null
        }
    }

    pub fn match_to_u8(&self, key: KeyCode) -> u8 {
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

    pub fn get_all_keys(&self) -> Vec<KeyCode> {
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
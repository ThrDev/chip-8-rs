use minifb::Key;

pub struct KeyMap {
}

impl KeyMap {
    pub fn match_to_key(key: u8) -> Key {
        match key {
            0x0 => Key::X,
            0x1 => Key::Key1,
            0x2 => Key::Key2,
            0x3 => Key::Key3,
            0x4 => Key::Q,
            0x5 => Key::W,
            0x6 => Key::E,
            0x7 => Key::A,
            0x8 => Key::S,
            0x9 => Key::D,
            0xA => Key::Z,
            0xB => Key::C,
            0xC => Key::Key4,
            0xD => Key::R,
            0xE => Key::F,
            0xF => Key::V,
            _ => Key::Unknown
        }
    }

    pub fn match_to_u8(key: Key) -> u8 {
        match key {
            x if Key::X == key => 0x0,
            x if Key::Key1 == key => 0x1,
            x if Key::Key2 == key => 0x2,
            x if Key::Key3 == key => 0x3,
            x if Key::Q == key => 0x4,
            x if Key::W == key => 0x5,
            x if Key::E == key => 0x6,
            x if Key::A == key => 0x7,
            x if Key::S == key => 0x8,
            x if Key::D == key => 0x9,
            x if Key::Z == key => 0xA,
            x if Key::C == key => 0xB,
            x if Key::Key4 == key => 0xC,
            x if Key::R == key => 0xD,
            x if Key::F == key => 0xE,
            x if Key::V == key => 0xF,
            _ => 0xFF
        }
    }

    pub fn get_all_keys() -> Vec<Key> {
        return vec!(Key::X,
                    Key::Key1,
                    Key::Key2,
                    Key::Key3,
                    Key::Q,
                    Key::W,
                    Key::E,
                    Key::A,
                    Key::S,
                    Key::D,
                    Key::Z,
                    Key::C,
                    Key::Key4,
                    Key::R,
                    Key::F,
                    Key::V);
    }
}
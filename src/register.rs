pub struct Register {
    pub v0: u8,
    pub v1: u8,
    pub v2: u8,
    pub v3: u8,
    pub v4: u8,
    pub v5: u8,
    pub v6: u8,
    pub v7: u8,
    pub v8: u8,
    pub v9: u8,
    pub va: u8,
    pub vb: u8,
    pub vc: u8,
    pub vd: u8,
    pub ve: u8,
    pub vf: u8,
}

impl Register {
    pub fn set_register(&mut self, register: u16, value: u8) {
        match register {
            0x0 => self.v0 = value,
            0x1 => self.v1 = value,
            0x2 => self.v2 = value,
            0x3 => self.v3 = value,
            0x4 => self.v4 = value,
            0x5 => self.v5 = value,
            0x6 => self.v6 = value,
            0x7 => self.v7 = value,
            0x8 => self.v8 = value,
            0x9 => self.v9 = value,
            0xa => self.va = value,
            0xb => self.vb = value,
            0xc => self.vc = value,
            0xd => self.vd = value,
            0xe => self.ve = value,
            0xf => self.vf = value,
            _ => ()
        }
    }

    pub fn get_register(&mut self, register: u16) -> u8 {
        match register {
            0x0 => self.v0,
            0x1 => self.v1,
            0x2 => self.v2,
            0x3 => self.v3,
            0x4 => self.v4,
            0x5 => self.v5,
            0x6 => self.v6,
            0x7 => self.v7,
            0x8 => self.v8,
            0x9 => self.v9,
            0xa => self.va,
            0xb => self.vb,
            0xc => self.vc,
            0xd => self.vd,
            0xe => self.ve,
            0xf => self.vf,
            _ => 0
        }
    }
}
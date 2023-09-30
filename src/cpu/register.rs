// https://gbdev.io/pandocs/CPU_Registers_and_Flags.html
pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

pub const ZERO_FLAG: u8 = 0b1000_0000;
pub const SUBTRACT_FLAG: u8 = 0b0100_0000;
pub const HALF_CARRY_FLAG: u8 = 0b0010_0000;
pub const CARRY_FLAG: u8 = 0b0001_0000;

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0x01,
            f: ZERO_FLAG | HALF_CARRY_FLAG | CARRY_FLAG,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xd8,
            h: 0x01,
            l: 0x4d,
            pc: 0x0100,
            sp: 0xfffe,
        }
    }

    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_flag(&mut self, flag: u8, is_set: bool) {
        if is_set {
            self.f |= flag;
        } else {
            self.f &= !flag;
        }
    }
}

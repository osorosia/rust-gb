// https://gbdev.io/pandocs/CPU_Registers_and_Flags.html
pub struct Registers {
    pub a: u8,
    pub f: RegisterFlag,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

pub struct RegisterFlag {
    pub zero: bool,
    pub subtraction: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0x01,
            f: RegisterFlag::new(),
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
}

impl RegisterFlag {
    pub fn new() -> Self {
        Self {
            zero: true,
            subtraction: false,
            half_carry: true,
            carry: true,
        }
    }
}

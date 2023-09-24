use crate::register::Registers;

pub struct Cpu {
    registers: Registers,
}


impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
    }
}

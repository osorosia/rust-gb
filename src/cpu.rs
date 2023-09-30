mod opcode;
mod register;

use register::Registers;

pub struct Cpu {
    reg: Registers,
    memory: [u8; 0xffff],
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            reg: Registers::new(),
            memory: [0; 0xffff],
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        if cfg!(debug_assertions) {
            println!("ROM size: {} bytes", rom.len());
        }

        for (i, byte) in rom.iter().enumerate() {
            self.memory[i + 0x100] = *byte;
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.fetch_byte();
            self.run_opcode(opcode);
        }
    }

    pub(crate) fn fetch_byte(&mut self) -> u8 {
        let opcode = self.read_u8(self.reg.pc);
        self.reg.pc += 1;
        opcode
    }

    pub(crate) fn read_u8(&mut self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub(crate) fn write_u8(&mut self, addr: u16, byte: u8) {
        self.memory[addr as usize] = byte;
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_hello() {
        let add = |a, b| a + b;
        assert_eq!(2, add(1, 1));
    }
}

use crate::register::Registers;

pub struct Cpu {
    registers: Registers,
    memory: [u8; 0xffff],
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
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

            match opcode {
                0xF0 => print!("LDH A, (a8)"),
                _ => print!("Unknown opcode: {:02X}", opcode),
            };
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let opcode = self.memory[self.registers.pc as usize];
        self.registers.pc += 1;
        opcode
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

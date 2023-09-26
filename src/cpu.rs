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
                0x00 => {
                    println!("NOP");
                }
                0xF0 => {
                    let a = 0xFF00 | self.fetch_byte() as u16;
                    self.registers.a = self.memory[a as usize];
                    println!("LDH A, 0x{:02X}", a);
                }
                0xFE => {
                    let n = self.fetch_byte();
                    self.registers.f.zero = self.registers.a == n;
                    self.registers.f.subtraction = true;
                    self.registers.f.half_carry = (self.registers.a & 0x0F) < (n & 0x0F);
                    self.registers.f.carry = self.registers.a < n;
                    println!("CP 0x{:02X}", n);
                }
                _ => panic!("Unknown opcode: 0x{:02X}", opcode),
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

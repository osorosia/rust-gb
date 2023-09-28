use once_cell::sync::Lazy;
use std::collections::HashMap;

use super::Cpu;

pub struct Opcode {
    pub typ: OpType,
    pub bytes: u8,
    pub cycles: u8,
    pub name: String,
}

impl Opcode {
    pub fn new(typ: OpType, bytes: u8, cycles: u8, name: &str) -> Self {
        let name = name.to_string();
        Self {
            typ,
            name,
            bytes,
            cycles,
        }
    }
}

pub enum OpType {
    Load,
    ALU,
    Misc,
    RotShift,
    BitOp,
    Jump,
    Call,
    Return,
    Invalid,
}

#[rustfmt::skip]
pub static OPCODE_DATA: Lazy<HashMap<u8, Opcode>> = Lazy::new(|| {
    let mut map = HashMap::<u8, Opcode>::new();

    map.insert(0x00, Opcode::new(OpType::Misc,     1, 4,  "NOP"));
    map.insert(0x01, Opcode::new(OpType::Load,     3, 12, "LD BC, nn"));
    map.insert(0x02, Opcode::new(OpType::Load,     1, 8,  "LD (BC), A"));
    map.insert(0x03, Opcode::new(OpType::Misc,     1, 8,  "INC BC"));
    map.insert(0x04, Opcode::new(OpType::ALU,      1, 4,  "INC B"));
    map.insert(0x05, Opcode::new(OpType::ALU,      1, 4,  "DEC B"));
    map.insert(0x06, Opcode::new(OpType::Load,     2, 8,  "LD B, n"));
    map.insert(0x07, Opcode::new(OpType::RotShift, 1, 4,  "RLCA"));
    map.insert(0x08, Opcode::new(OpType::Load,     3, 20, "LD (nn), SP"));
    map.insert(0x09, Opcode::new(OpType::ALU,      1, 8,  "ADD HL, BC"));
    map.insert(0x0A, Opcode::new(OpType::Load,     1, 8,  "LD A, (BC)"));
    map.insert(0x0B, Opcode::new(OpType::Misc,     1, 8,  "DEC BC"));
    map.insert(0x0C, Opcode::new(OpType::ALU,      1, 4,  "INC C"));
    map.insert(0x0D, Opcode::new(OpType::ALU,      1, 4,  "DEC C"));
    map.insert(0x0E, Opcode::new(OpType::Load,     2, 8,  "LD C, n"));
    map.insert(0x0F, Opcode::new(OpType::RotShift, 1, 4,  "RRCA"));
    map.insert(0x10, Opcode::new(OpType::Misc,     1, 4,  "STOP"));
    map.insert(0x11, Opcode::new(OpType::Load,     3, 12, "LD DE, nn"));
    map.insert(0x12, Opcode::new(OpType::Load,     1, 8,  "LD (DE), A"));
    map.insert(0x13, Opcode::new(OpType::Misc,     1, 8,  "INC DE"));
    map.insert(0x14, Opcode::new(OpType::ALU,      1, 4,  "INC D"));
    map.insert(0x15, Opcode::new(OpType::ALU,      1, 4,  "DEC D"));
    map.insert(0x16, Opcode::new(OpType::Load,     2, 8,  "LD D, n"));
    map.insert(0x17, Opcode::new(OpType::RotShift, 1, 4,  "RLA"));
    map.insert(0x18, Opcode::new(OpType::Jump,     2, 12, "JR n"));
    map.insert(0x19, Opcode::new(OpType::ALU,      1, 8,  "ADD HL, DE"));
    map.insert(0x1A, Opcode::new(OpType::Load,     1, 8,  "LD A, (DE)"));
    map.insert(0x1B, Opcode::new(OpType::Misc,     1, 8,  "DEC DE"));
    map.insert(0x1C, Opcode::new(OpType::ALU,      1, 4,  "INC E"));
    map.insert(0x1D, Opcode::new(OpType::ALU,      1, 4,  "DEC E"));
    map.insert(0x1E, Opcode::new(OpType::Load,     2, 8,  "LD E, n"));
    map.insert(0x1F, Opcode::new(OpType::RotShift, 1, 4,  "RRA"));
    map.insert(0x20, Opcode::new(OpType::Jump,     2, 8,  "JR NZ, n"));
    map.insert(0x21, Opcode::new(OpType::Load,     3, 12, "LD HL, nn"));
    map.insert(0x22, Opcode::new(OpType::Load,     1, 8,  "LD (HL+), A"));
    map.insert(0x23, Opcode::new(OpType::Misc,     1, 8,  "INC HL"));
    map.insert(0x24, Opcode::new(OpType::ALU,      1, 4,  "INC H"));
    map.insert(0x25, Opcode::new(OpType::ALU,      1, 4,  "DEC H"));
    map.insert(0x26, Opcode::new(OpType::Load,     2, 8,  "LD H, n"));
    map.insert(0x27, Opcode::new(OpType::RotShift, 1, 4,  "DAA"));
    map.insert(0x28, Opcode::new(OpType::Jump,     2, 8,  "JR Z, n"));
    map.insert(0x29, Opcode::new(OpType::ALU,      1, 8,  "ADD HL, HL"));
    map.insert(0x2A, Opcode::new(OpType::Load,     1, 8,  "LD A, (HL+)"));
    map.insert(0x2B, Opcode::new(OpType::Misc,     1, 8,  "DEC HL"));
    map.insert(0x2C, Opcode::new(OpType::ALU,      1, 4,  "INC L"));
    map.insert(0x2D, Opcode::new(OpType::ALU,      1, 4,  "DEC L"));
    map.insert(0x2E, Opcode::new(OpType::Load,     2, 8,  "LD L, n"));
    map.insert(0x2F, Opcode::new(OpType::RotShift, 1, 4,  "CPL"));
    map.insert(0x30, Opcode::new(OpType::Jump,     2, 8,  "JR NC, n"));
    map.insert(0x31, Opcode::new(OpType::Load,     3, 12, "LD SP, nn"));
    map.insert(0x32, Opcode::new(OpType::Load,     1, 8,  "LD (HL-), A"));
    map.insert(0x33, Opcode::new(OpType::Misc,     1, 8,  "INC SP"));
    map.insert(0x34, Opcode::new(OpType::ALU,      1, 12, "INC (HL)"));
    map.insert(0x35, Opcode::new(OpType::ALU,      1, 12, "DEC (HL)"));
    map.insert(0x36, Opcode::new(OpType::Load,     2, 12, "LD (HL), n"));
    map.insert(0x37, Opcode::new(OpType::RotShift, 1, 4,  "SCF"));
    map.insert(0x38, Opcode::new(OpType::Jump,     2, 8,  "JR C, n"));
    map.insert(0x39, Opcode::new(OpType::ALU,      1, 8,  "ADD HL, SP"));
    map.insert(0x3A, Opcode::new(OpType::Load,     1, 8,  "LD A, (HL-)"));
    map.insert(0x3B, Opcode::new(OpType::Misc,     1, 8,  "DEC SP"));
    map.insert(0x3C, Opcode::new(OpType::ALU,      1, 4,  "INC A"));
    map.insert(0x3D, Opcode::new(OpType::ALU,      1, 4,  "DEC A"));
    map.insert(0x3E, Opcode::new(OpType::Load,     2, 8,  "LD A, n"));
    map.insert(0x3F, Opcode::new(OpType::RotShift, 1, 4,  "CCF"));
    map.insert(0x40, Opcode::new(OpType::Load,     1, 4,  "LD B, B"));
    map.insert(0x41, Opcode::new(OpType::Load,     1, 4,  "LD B, C"));
    map.insert(0x42, Opcode::new(OpType::Load,     1, 4,  "LD B, D"));
    map.insert(0x43, Opcode::new(OpType::Load,     1, 4,  "LD B, E"));
    map.insert(0x44, Opcode::new(OpType::Load,     1, 4,  "LD B, H"));
    map.insert(0x45, Opcode::new(OpType::Load,     1, 4,  "LD B, L"));
    map.insert(0x46, Opcode::new(OpType::Load,     1, 8,  "LD B, (HL)"));
    map.insert(0x47, Opcode::new(OpType::Load,     1, 4,  "LD B, A"));
    map.insert(0x48, Opcode::new(OpType::Load,     1, 4,  "LD C, B"));
    map.insert(0x49, Opcode::new(OpType::Load,     1, 4,  "LD C, C"));
    map.insert(0x4A, Opcode::new(OpType::Load,     1, 4,  "LD C, D"));
    map.insert(0x4B, Opcode::new(OpType::Load,     1, 4,  "LD C, E"));
    map.insert(0x4C, Opcode::new(OpType::Load,     1, 4,  "LD C, H"));
    map.insert(0x4D, Opcode::new(OpType::Load,     1, 4,  "LD C, L"));
    map.insert(0x4E, Opcode::new(OpType::Load,     1, 8,  "LD C, (HL)"));
    map.insert(0x4F, Opcode::new(OpType::Load,     1, 4,  "LD C, A"));
    map.insert(0x50, Opcode::new(OpType::Load,     1, 4,  "LD D, B"));
    map.insert(0x51, Opcode::new(OpType::Load,     1, 4,  "LD D, C"));
    map.insert(0x52, Opcode::new(OpType::Load,     1, 4,  "LD D, D"));
    map.insert(0x53, Opcode::new(OpType::Load,     1, 4,  "LD D, E"));
    map.insert(0x54, Opcode::new(OpType::Load,     1, 4,  "LD D, H"));
    map.insert(0x55, Opcode::new(OpType::Load,     1, 4,  "LD D, L"));
    map.insert(0x56, Opcode::new(OpType::Load,     1, 8,  "LD D, (HL)"));
    map.insert(0x57, Opcode::new(OpType::Load,     1, 4,  "LD D, A"));
    map.insert(0x58, Opcode::new(OpType::Load,     1, 4,  "LD E, B"));
    map.insert(0x59, Opcode::new(OpType::Load,     1, 4,  "LD E, C"));
    map.insert(0x5A, Opcode::new(OpType::Load,     1, 4,  "LD E, D"));
    map.insert(0x5B, Opcode::new(OpType::Load,     1, 4,  "LD E, E"));
    map.insert(0x5C, Opcode::new(OpType::Load,     1, 4,  "LD E, H"));
    map.insert(0x5D, Opcode::new(OpType::Load,     1, 4,  "LD E, L"));
    map.insert(0x5E, Opcode::new(OpType::Load,     1, 8,  "LD E, (HL)"));
    map.insert(0x5F, Opcode::new(OpType::Load,     1, 4,  "LD E, A"));
    map.insert(0x60, Opcode::new(OpType::Load,     1, 4,  "LD H, B"));
    map.insert(0x61, Opcode::new(OpType::Load,     1, 4,  "LD H, C"));
    map.insert(0x62, Opcode::new(OpType::Load,     1, 4,  "LD H, D"));
    map.insert(0x63, Opcode::new(OpType::Load,     1, 4,  "LD H, E"));
    map.insert(0x64, Opcode::new(OpType::Load,     1, 4,  "LD H, H"));
    map.insert(0x65, Opcode::new(OpType::Load,     1, 4,  "LD H, L"));
    map.insert(0x66, Opcode::new(OpType::Load,     1, 8,  "LD H, (HL)"));
    map.insert(0x67, Opcode::new(OpType::Load,     1, 4,  "LD H, A"));
    map.insert(0x68, Opcode::new(OpType::Load,     1, 4,  "LD L, B"));
    map.insert(0x69, Opcode::new(OpType::Load,     1, 4,  "LD L, C"));
    map.insert(0x6A, Opcode::new(OpType::Load,     1, 4,  "LD L, D"));
    map.insert(0x6B, Opcode::new(OpType::Load,     1, 4,  "LD L, E"));
    map.insert(0x6C, Opcode::new(OpType::Load,     1, 4,  "LD L, H"));
    map.insert(0x6D, Opcode::new(OpType::Load,     1, 4,  "LD L, L"));
    map.insert(0x6E, Opcode::new(OpType::Load,     1, 8,  "LD L, (HL)"));
    map.insert(0x6F, Opcode::new(OpType::Load,     1, 4,  "LD L, A"));
    map.insert(0x70, Opcode::new(OpType::Load,     1, 8,  "LD (HL), B"));
    map.insert(0x71, Opcode::new(OpType::Load,     1, 8,  "LD (HL), C"));
    map.insert(0x72, Opcode::new(OpType::Load,     1, 8,  "LD (HL), D"));
    map.insert(0x73, Opcode::new(OpType::Load,     1, 8,  "LD (HL), E"));
    map.insert(0x74, Opcode::new(OpType::Load,     1, 8,  "LD (HL), H"));
    map.insert(0x75, Opcode::new(OpType::Load,     1, 8,  "LD (HL), L"));
    map.insert(0x76, Opcode::new(OpType::Misc,     1, 4,  "HALT"));
    map.insert(0x77, Opcode::new(OpType::Load,     1, 8,  "LD (HL), A"));
    map.insert(0x78, Opcode::new(OpType::Load,     1, 4,  "LD A, B"));
    map.insert(0x79, Opcode::new(OpType::Load,     1, 4,  "LD A, C"));
    map.insert(0x7A, Opcode::new(OpType::Load,     1, 4,  "LD A, D"));
    map.insert(0x7B, Opcode::new(OpType::Load,     1, 4,  "LD A, E"));
    map.insert(0x7C, Opcode::new(OpType::Load,     1, 4,  "LD A, H"));
    map.insert(0x7D, Opcode::new(OpType::Load,     1, 4,  "LD A, L"));
    map.insert(0x7E, Opcode::new(OpType::Load,     1, 8,  "LD A, (HL)"));
    map.insert(0x7F, Opcode::new(OpType::Load,     1, 4,  "LD A, A"));
    map.insert(0x80, Opcode::new(OpType::ALU,      1, 4,  "ADD A, B"));
    map.insert(0x81, Opcode::new(OpType::ALU,      1, 4,  "ADD A, C"));
    map.insert(0x82, Opcode::new(OpType::ALU,      1, 4,  "ADD A, D"));
    map.insert(0x83, Opcode::new(OpType::ALU,      1, 4,  "ADD A, E"));
    map.insert(0x84, Opcode::new(OpType::ALU,      1, 4,  "ADD A, H"));
    map.insert(0x85, Opcode::new(OpType::ALU,      1, 4,  "ADD A, L"));
    map.insert(0x86, Opcode::new(OpType::ALU,      1, 8,  "ADD A, (HL)"));
    map.insert(0x87, Opcode::new(OpType::ALU,      1, 4,  "ADD A, A"));
    map.insert(0x88, Opcode::new(OpType::ALU,      1, 4,  "ADC A, B"));
    map.insert(0x89, Opcode::new(OpType::ALU,      1, 4,  "ADC A, C"));
    map.insert(0x8A, Opcode::new(OpType::ALU,      1, 4,  "ADC A, D"));
    map.insert(0x8B, Opcode::new(OpType::ALU,      1, 4,  "ADC A, E"));
    map.insert(0x8C, Opcode::new(OpType::ALU,      1, 4,  "ADC A, H"));
    map.insert(0x8D, Opcode::new(OpType::ALU,      1, 4,  "ADC A, L"));
    map.insert(0x8E, Opcode::new(OpType::ALU,      1, 8,  "ADC A, (HL)"));
    map.insert(0x8F, Opcode::new(OpType::ALU,      1, 4,  "ADC A, A"));
    map.insert(0x90, Opcode::new(OpType::ALU,      1, 4,  "SUB B"));
    map.insert(0x91, Opcode::new(OpType::ALU,      1, 4,  "SUB C"));
    map.insert(0x92, Opcode::new(OpType::ALU,      1, 4,  "SUB D"));
    map.insert(0x93, Opcode::new(OpType::ALU,      1, 4,  "SUB E"));
    map.insert(0x94, Opcode::new(OpType::ALU,      1, 4,  "SUB H"));
    map.insert(0x95, Opcode::new(OpType::ALU,      1, 4,  "SUB L"));
    map.insert(0x96, Opcode::new(OpType::ALU,      1, 8,  "SUB (HL)"));
    map.insert(0x97, Opcode::new(OpType::ALU,      1, 4,  "SUB A"));
    map.insert(0x98, Opcode::new(OpType::ALU,      1, 4,  "SBC A, B"));
    map.insert(0x99, Opcode::new(OpType::ALU,      1, 4,  "SBC A, C"));
    map.insert(0x9A, Opcode::new(OpType::ALU,      1, 4,  "SBC A, D"));
    map.insert(0x9B, Opcode::new(OpType::ALU,      1, 4,  "SBC A, E"));
    map.insert(0x9C, Opcode::new(OpType::ALU,      1, 4,  "SBC A, H"));
    map.insert(0x9D, Opcode::new(OpType::ALU,      1, 4,  "SBC A, L"));
    map.insert(0x9E, Opcode::new(OpType::ALU,      1, 8,  "SBC A, (HL)"));
    map.insert(0x9F, Opcode::new(OpType::ALU,      1, 4,  "SBC A, A"));
    map.insert(0xA0, Opcode::new(OpType::ALU,      1, 4,  "AND B"));
    map.insert(0xA1, Opcode::new(OpType::ALU,      1, 4,  "AND C"));
    map.insert(0xA2, Opcode::new(OpType::ALU,      1, 4,  "AND D"));
    map.insert(0xA3, Opcode::new(OpType::ALU,      1, 4,  "AND E"));
    map.insert(0xA4, Opcode::new(OpType::ALU,      1, 4,  "AND H"));
    map.insert(0xA5, Opcode::new(OpType::ALU,      1, 4,  "AND L"));
    map.insert(0xA6, Opcode::new(OpType::ALU,      1, 8,  "AND (HL)"));
    map.insert(0xA7, Opcode::new(OpType::ALU,      1, 4,  "AND A"));
    map.insert(0xA8, Opcode::new(OpType::ALU,      1, 4,  "XOR B"));
    map.insert(0xA9, Opcode::new(OpType::ALU,      1, 4,  "XOR C"));
    map.insert(0xAA, Opcode::new(OpType::ALU,      1, 4,  "XOR D"));
    map.insert(0xAB, Opcode::new(OpType::ALU,      1, 4,  "XOR E"));
    map.insert(0xAC, Opcode::new(OpType::ALU,      1, 4,  "XOR H"));
    map.insert(0xAD, Opcode::new(OpType::ALU,      1, 4,  "XOR L"));
    map.insert(0xAE, Opcode::new(OpType::ALU,      1, 8,  "XOR (HL)"));
    map.insert(0xAF, Opcode::new(OpType::ALU,      1, 4,  "XOR A"));
    map.insert(0xB0, Opcode::new(OpType::ALU,      1, 4,  "OR B"));
    map.insert(0xB1, Opcode::new(OpType::ALU,      1, 4,  "OR C"));
    map.insert(0xB2, Opcode::new(OpType::ALU,      1, 4,  "OR D"));
    map.insert(0xB3, Opcode::new(OpType::ALU,      1, 4,  "OR E"));
    map.insert(0xB4, Opcode::new(OpType::ALU,      1, 4,  "OR H"));
    map.insert(0xB5, Opcode::new(OpType::ALU,      1, 4,  "OR L"));
    map.insert(0xB6, Opcode::new(OpType::ALU,      1, 8,  "OR (HL)"));
    map.insert(0xB7, Opcode::new(OpType::ALU,      1, 4,  "OR A"));
    map.insert(0xB8, Opcode::new(OpType::ALU,      1, 4,  "CP B"));
    map.insert(0xB9, Opcode::new(OpType::ALU,      1, 4,  "CP C"));
    map.insert(0xBA, Opcode::new(OpType::ALU,      1, 4,  "CP D"));
    map.insert(0xBB, Opcode::new(OpType::ALU,      1, 4,  "CP E"));
    map.insert(0xBC, Opcode::new(OpType::ALU,      1, 4,  "CP H"));
    map.insert(0xBD, Opcode::new(OpType::ALU,      1, 4,  "CP L"));
    map.insert(0xBE, Opcode::new(OpType::ALU,      1, 8,  "CP (HL)"));
    map.insert(0xBF, Opcode::new(OpType::ALU,      1, 4,  "CP A"));
    map.insert(0xC0, Opcode::new(OpType::Return,   1, 8,  "RET NZ"));
    map.insert(0xC1, Opcode::new(OpType::Load,     1, 12, "POP BC"));
    map.insert(0xC2, Opcode::new(OpType::Jump,     3, 12, "JP NZ, nn"));
    map.insert(0xC3, Opcode::new(OpType::Jump,     3, 16, "JP nn"));
    map.insert(0xC4, Opcode::new(OpType::Call,     3, 12, "CALL NZ, nn"));
    map.insert(0xC5, Opcode::new(OpType::Load,     1, 16, "PUSH BC"));
    map.insert(0xC6, Opcode::new(OpType::ALU,      2, 8,  "ADD A, n"));
    map.insert(0xC7, Opcode::new(OpType::Jump,     1, 16, "RST 00H"));
    map.insert(0xC8, Opcode::new(OpType::Return,   1, 8,  "RET Z"));
    map.insert(0xC9, Opcode::new(OpType::Return,   1, 16, "RET"));
    map.insert(0xCA, Opcode::new(OpType::Jump,     3, 12, "JP Z, nn"));
    map.insert(0xCB, Opcode::new(OpType::BitOp,    2, 8,  "PREFIX CB"));
    map.insert(0xCC, Opcode::new(OpType::Call,     3, 12, "CALL Z, nn"));
    map.insert(0xCD, Opcode::new(OpType::Call,     3, 24, "CALL nn"));
    map.insert(0xCE, Opcode::new(OpType::ALU,      2, 8,  "ADC A, n"));
    map.insert(0xCF, Opcode::new(OpType::Jump,     1, 16, "RST 08H"));
    map.insert(0xD0, Opcode::new(OpType::Return,   1, 8,  "RET NC"));
    map.insert(0xD1, Opcode::new(OpType::Load,     1, 12, "POP DE"));
    map.insert(0xD2, Opcode::new(OpType::Jump,     3, 12, "JP NC, nn"));
    map.insert(0xD3, Opcode::new(OpType::Invalid,  2, 4,  "INVALID"));
    map.insert(0xD4, Opcode::new(OpType::Call,     3, 12, "CALL NC, nn"));
    map.insert(0xD5, Opcode::new(OpType::Load,     1, 16, "PUSH DE"));
    map.insert(0xD6, Opcode::new(OpType::ALU,      2, 8,  "SUB n"));
    map.insert(0xD7, Opcode::new(OpType::Jump,     1, 16, "RST 10H"));
    map.insert(0xD8, Opcode::new(OpType::Return,   1, 8,  "RET C"));
    map.insert(0xD9, Opcode::new(OpType::Return,   1, 16, "RETI"));
    map.insert(0xDA, Opcode::new(OpType::Jump,     3, 12, "JP C, nn"));
    map.insert(0xDB, Opcode::new(OpType::Invalid,  2, 4,  "INVALID"));
    map.insert(0xDC, Opcode::new(OpType::Call,     3, 12, "CALL C, nn"));
    map.insert(0xDD, Opcode::new(OpType::Invalid,  2, 4,  "INVALID"));
    map.insert(0xDE, Opcode::new(OpType::ALU,      2, 8,  "SBC A, n"));
    map.insert(0xDF, Opcode::new(OpType::Jump,     1, 16, "RST 18H"));
    map.insert(0xE0, Opcode::new(OpType::Load,     2, 12, "LDH (n), A"));
    map.insert(0xE1, Opcode::new(OpType::Load,     1, 12, "POP HL"));
    map.insert(0xE2, Opcode::new(OpType::Load,     1, 8,  "LD (C), A"));
    map.insert(0xE3, Opcode::new(OpType::Invalid,  1, 4,  "INVALID"));
    map.insert(0xE4, Opcode::new(OpType::Invalid,  1, 4,  "INVALID"));
    map.insert(0xE5, Opcode::new(OpType::Load,     1, 16, "PUSH HL"));
    map.insert(0xE6, Opcode::new(OpType::ALU,      2, 8,  "AND n"));
    map.insert(0xE7, Opcode::new(OpType::Jump,     1, 16, "RST 20H"));
    map.insert(0xE8, Opcode::new(OpType::Load,     2, 16, "ADD SP, n"));
    map.insert(0xE9, Opcode::new(OpType::Jump,     1, 4,  "JP (HL)"));
    map.insert(0xEA, Opcode::new(OpType::Load,     3, 16, "LD (nn), A"));
    map.insert(0xEB, Opcode::new(OpType::Invalid,  1, 4,  "INVALID"));
    map.insert(0xEC, Opcode::new(OpType::Invalid,  1, 4,  "INVALID"));
    map.insert(0xED, Opcode::new(OpType::Invalid,  1, 4,  "INVALID"));
    map.insert(0xEE, Opcode::new(OpType::ALU,      2, 8,  "XOR n"));
    map.insert(0xEF, Opcode::new(OpType::Jump,     1, 16, "RST 28H"));
    map.insert(0xF0, Opcode::new(OpType::Load,     2, 12, "LDH A, (n)"));
    map.insert(0xF1, Opcode::new(OpType::Load,     1, 12, "POP AF"));
    map.insert(0xF2, Opcode::new(OpType::Load,     1, 8,  "LD A, (C)"));
    map.insert(0xF3, Opcode::new(OpType::Misc,     1, 4,  "DI"));
    map.insert(0xF4, Opcode::new(OpType::Invalid,  1, 4,  "INVALID"));
    map.insert(0xF5, Opcode::new(OpType::Load,     1, 16, "PUSH AF"));
    map.insert(0xF6, Opcode::new(OpType::ALU,      2, 8,  "OR n"));
    map.insert(0xF7, Opcode::new(OpType::Jump,     1, 16, "RST 30H"));
    map.insert(0xF8, Opcode::new(OpType::Load,     2, 12, "LD HL, SP+n"));
    map.insert(0xF9, Opcode::new(OpType::Load,     1, 8,  "LD SP, HL"));
    map.insert(0xFA, Opcode::new(OpType::Load,     3, 16, "LD A, (nn)"));
    map.insert(0xFB, Opcode::new(OpType::Misc,     1, 4,  "EI"));
    map.insert(0xFC, Opcode::new(OpType::Invalid,  1, 4,  "INVALID"));
    map.insert(0xFD, Opcode::new(OpType::Invalid,  1, 4,  "INVALID"));
    map.insert(0xFE, Opcode::new(OpType::ALU,      2, 8,  "CP n"));
    map.insert(0xFF, Opcode::new(OpType::Jump,     1, 16, "RST 38H"));

    map
});

impl Cpu {
    pub(crate) fn run_opcode(&mut self, opcode: u8) {
        let op = OPCODE_DATA.get(&opcode).unwrap();

        match op.typ {
            _ => panic!("Unknown opcode: 0x{:02X}", opcode),
        };
    }

    fn op_load(&mut self, opcode: u8) {
        match opcode {
            _ => panic!("Unknown opcode: 0x{:02X}", opcode),
        }
    }
}

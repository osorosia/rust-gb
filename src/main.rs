pub mod cpu;
pub mod register;

use cpu::Cpu;
use std::env;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    if args.len() == 0 {
        println!("No ROM file specified");
        return;
    }

    let rom = std::fs::read(&args[0]).unwrap();

    let mut cpu = Cpu::new();
    cpu.load_rom(rom);

    // cpu.run();
}

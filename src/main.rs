pub mod cpu;
pub mod register;

use cpu::Cpu;
use std::env;

fn add(a: &mut u8, b: u8) {
    let c = *a + b;
    *a = c;
}

fn main() {
    let mut a = 1;
    add(&mut a, 1);
    println!("a: {}", a);

    let args = env::args().skip(1).collect::<Vec<String>>();
    if args.is_empty() {
        println!("No ROM file specified");
        return;
    }

    let rom = std::fs::read(&args[0]).unwrap();

    let mut cpu = Cpu::new();

    cpu.load_rom(rom);

    cpu.run();
}

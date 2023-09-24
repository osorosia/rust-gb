pub mod cpu;
pub mod register;

use cpu::Cpu;

fn main() {
    println!("Hello, world!");
    let cpu = Cpu::new();
}

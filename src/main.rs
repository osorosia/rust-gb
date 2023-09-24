pub mod cpu;
pub mod register;

use cpu::Cpu;
use std::env;

fn main() {
    println!("Hello, world!");
    let cpu = Cpu::new();

    let args = env::args().skip(1).collect::<Vec<String>>();

    if args.len() == 0 {
        println!("No ROM file specified");
        return;
    }

    println!("{:?}", args);
}

mod instructions;

mod cpu;
use cpu::CPU;

/*const IP: u8 = 0;
const GRA: u8 = 1;
const GRB: u8 = 2;
const GRC: u8 = 3;
const GRD: u8 = 4;
const RRA: u8 = 5;*/

fn main() {
    let mut path = String::new();

    match std::io::stdin().read_line(&mut path){
        Ok(path) => path,
        Err(error) => panic!("Cannot Input Path: {}", error)
    };

    let program_memory = match std::fs::read(path.trim()) {
        Ok(file) => file,
        Err(error) => panic!("Cannot Read File: {}", error)
    };

    /*for memory in program_memory.clone() {
        print!("{:#04X} ", memory);
    }*/

    let memory: Vec<u8> = program_memory.iter()
        .cloned()
        .chain(std::iter::repeat(0x00).take(256*256 - program_memory.len()))
        .collect();

    let mut cpu = CPU::new(memory);

    cpu.run();
}
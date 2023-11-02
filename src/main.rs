mod instructions;
use instructions::instructions::{MOV_REG_LIT, ADD_REG_REG};

mod cpu;
use cpu::CPU;

fn main() {
    let program_memory: Vec<u8> = vec![MOV_REG_LIT, 1, 0x12, 0x34, MOV_REG_LIT, 2, 0xAB, 0xCD, ADD_REG_REG, 1, 2]; //vec![MOV_GRA_LIT, 0x12, 0x34, MOV_GRB_LIT, 0xAB, 0xCD, ADD_REG_REG, 1, 2];

    let mut cpu = CPU::new(program_memory);

    cpu.debug();

    cpu.step();
    cpu.debug();

    cpu.step();
    cpu.debug();

    cpu.step();
    cpu.debug();
}
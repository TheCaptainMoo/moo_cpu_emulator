mod instructions;
use crate::instructions::instructions::{MOV_GRA_LIT, MOV_GRB_LIT, ADD_REG_REG};

use cpu::CPU;
mod cpu;

fn main() {
    let program_memory: Vec<u8> = vec![MOV_GRA_LIT, 0x12, 0x34, MOV_GRB_LIT, 0xAB, 0xCD, ADD_REG_REG, 1, 2];

    let mut cpu = CPU::new(program_memory);

    cpu.debug();

    cpu.step();
    cpu.debug();

    cpu.step();
    cpu.debug();

    cpu.step();
    cpu.debug();
}
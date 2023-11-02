mod instructions;
use instructions::instructions::{MOV_LIT_REG, ADD_REG_REG, MOV_REG_MEM};

mod cpu;
use cpu::CPU;

const IP: u8 = 0;
const GRA: u8 = 1;
const GRB: u8 = 2;
const GRC: u8 = 3;
const GRD: u8 = 4;
const RRA: u8 = 5;

fn main() {
    let program_memory: Vec<u8> = vec![MOV_LIT_REG, GRA, 0x12, 0x34, MOV_LIT_REG, GRB, 0xAB, 0xCD, ADD_REG_REG, GRA, GRB, MOV_REG_MEM, RRA, 0x01, 0x00]; //vec![MOV_GRA_LIT, 0x12, 0x34, MOV_GRB_LIT, 0xAB, 0xCD, ADD_REG_REG, 1, 2];

    let memory: Vec<u8> = program_memory.iter()
        .cloned()
        .chain(std::iter::repeat(0x00).take(256*256 - program_memory.len()))
        .collect();

    let mut cpu = CPU::new(memory);

    cpu.debug();
    cpu.view_memory(0x0100);

    cpu.step();
    cpu.debug();
    cpu.view_memory(0x0100);

    cpu.step();
    cpu.debug();
    cpu.view_memory(0x0100);

    cpu.step();
    cpu.debug();
    cpu.view_memory(0x0100);

    cpu.step();
    cpu.debug();
    cpu.view_memory(0x0100);
}
mod instructions;
use instructions::instructions::{MOV_LIT_REG, ADD_REG_REG, MOV_REG_MEM, MOV_MEM_REG, JNE_LIT, HLT};

mod cpu;
use cpu::CPU;

const IP: u8 = 0;
const GRA: u8 = 1;
const GRB: u8 = 2;
const GRC: u8 = 3;
const GRD: u8 = 4;
const RRA: u8 = 5;

fn main() {
    // Counter from 0 to 3 example 
    let program_memory: Vec<u8> = vec![
        MOV_MEM_REG, 0x01, 0x00, GRA, 
        MOV_LIT_REG, 0x00, 0x01, GRB, 
        ADD_REG_REG, GRA, GRB,
        MOV_REG_MEM, RRA, 0x01, 0x00,
        JNE_LIT, 0x00, 0x03, 0x00, 0x00,
        HLT
    ];
    
    // Writing Register To Memory Example
    /*vec![
        MOV_LIT_REG, 0x12, 0x34, GRA, 
        MOV_LIT_REG, 0xAB, 0xCD, GRB,
        ADD_REG_REG, GRA, GRB, 
        MOV_REG_MEM, RRA, 0x01, 0x00,
        HLT
    ];*/

    let memory: Vec<u8> = program_memory.iter()
        .cloned()
        .chain(std::iter::repeat(0x00).take(256*256 - program_memory.len()))
        .collect();

    let mut cpu = CPU::new(memory);

    cpu.run();

    /*cpu.debug();
    cpu.view_memory(0x0100);

    for _ in 0..20 {
        cpu.step();
        cpu.debug();
        cpu.view_memory(0x0100);
    }*/

    /*cpu.step();
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
    cpu.view_memory(0x0100);*/
}
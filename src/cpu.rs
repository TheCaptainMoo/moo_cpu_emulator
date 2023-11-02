use std::collections::HashMap;
use crate::instructions::instructions::{ADD_REG_REG, MOV_REG_LIT};

const REGISTERS: [&str; 6] = ["ip", "gra", "grb", "grc", "grd", "rra"];

pub struct CPU<'a> {
    register_memory: Vec<u8>,
    register_map: HashMap<&'a str, usize>,
    program_memory: Vec<u8>
}

// Joins u8 and u8 together to make u16
fn bind_u8(byte1: u8, byte2: u8) -> u16 {
    ((byte1 as u16) << 8) | (byte2 as u16)
}

// Splits u16 into [u8; 2]
fn split_u16(joined_byte: u16) -> [u8; 2] {
    [(joined_byte >> 8) as u8, joined_byte as u8]
}

impl CPU<'_> {
    pub fn new(program_memory: Vec<u8>) -> CPU<'static> {
        CPU { 
            register_memory: vec![0; REGISTERS.len()*2],
            register_map: {
                let mut map = HashMap::new();
        
                let mut index: usize = 0;
                for register in REGISTERS {
                    map.insert(register, index);
                    index += 2;
                }
        
                map
            },
            program_memory
        }
    }

    fn write_register(&mut self, register: &str, data: u16){
        let index = match self.register_map.get(register) {
            Some(index) => index,
            None => panic!()
        };

        let data = split_u16(data);
    
        self.register_memory[*index] = data[0];
        self.register_memory[*index + 1] = data[1];
    }

    fn read_register(&mut self, register: &str) -> u16 {
        let index = match self.register_map.get(register) {
            Some(index) => index,
            None => panic!()
        };

        bind_u8(self.register_memory[*index], self.register_memory[*index + 1])
    }

    fn read_byte(&mut self, index: usize) -> u8 {
        self.program_memory[index]
    }

    fn read_dual_bytes(&mut self, index: usize) -> u16 {
        bind_u8(self.program_memory[index], self.program_memory[index + 1])
    }

    fn fetch(&mut self) -> u8 {
        let instruction_address = self.read_register("ip");
        let instruction = self.read_byte(instruction_address as usize);
        self.write_register("ip", instruction_address + 1);

        instruction
    }

    fn fetch_16(&mut self) -> u16 {
        let instruction_address = self.read_register("ip");
        let instruction = self.read_dual_bytes(instruction_address as usize);
        self.write_register("ip", instruction_address + 2);

        instruction
    }

    fn read_register_index(&mut self, index: usize) -> u16 {
        bind_u8(self.register_memory[index*2], self.register_memory[index*2 + 1])
    }

    fn write_register_index(&mut self, index: usize, data: u16){
        let data = split_u16(data);
    
        self.register_memory[index*2] = data[0];
        self.register_memory[index*2 + 1] = data[1];
    }

    fn execute(&mut self, instruction: u8) {
        match instruction {
            // Move Literal into Any Register
            MOV_REG_LIT => {
                let r1 = self.fetch();
                let literal = self.fetch_16();

                self.write_register_index(r1 as usize, literal);
            }

            // Add Registers Together
            ADD_REG_REG => {
                let gr1 = self.fetch();
                let gr2 = self.fetch();

                let gr1_value = self.read_register_index(gr1 as usize);
                let gr2_value = self.read_register_index(gr2 as usize);

                self.write_register("rra", gr1_value + gr2_value);
            }
            _ => {}
        }
    }

    pub fn step(&mut self) {
        let instruction = self.fetch();
        self.execute(instruction);
    }

    pub fn debug(&mut self) {
        for (index, byte) in self.register_memory.iter().enumerate() {
            println!("Memory[{}]: {:#04X}", index, byte);
        }

        print!("\n");
    }
}
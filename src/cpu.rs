use std::collections::HashMap;
use crate::instructions::instructions::{ADD_REG_REG, MOV_LIT_REG, MOV_REG_REG, MOV_MEM_REG, MOV_REG_MEM, JMP_NOT_EQ, HLT, MOV_LIT_MEM, ADD_LIT_REG, SUB_REG_REG, SUB_LIT_REG, SUB_REG_LIT, MUL_REG_REG, MUL_LIT_REG, INC_REG, DEC_REG};

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

    fn write_register_index(&mut self, index: usize, data: u16){
        let data = split_u16(data);
    
        self.register_memory[index*2] = data[0];
        self.register_memory[index*2 + 1] = data[1];
    }

    fn write_byte(&mut self, index: usize, data: u8){
        self.program_memory[index] = data;
    }

    fn write_dual_bytes(&mut self, index: usize, data: u16){
        let data = split_u16(data);

        self.program_memory[index] = data[0];
        self.program_memory[index + 1] = data[1];
    }

    fn read_register(&mut self, register: &str) -> u16 {
        let index = match self.register_map.get(register) {
            Some(index) => index,
            None => panic!()
        };

        bind_u8(self.register_memory[*index], self.register_memory[*index + 1])
    }

    fn read_register_index(&mut self, index: usize) -> u16 {
        bind_u8(self.register_memory[index*2], self.register_memory[index*2 + 1])
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

    fn execute(&mut self, instruction: u8) {
        match instruction {
            // Move Literal into Register
            MOV_LIT_REG => {
                let literal = self.fetch_16();
                let r1 = self.fetch();

                self.write_register_index(r1 as usize, literal);
            }

            // Move Register (r2) into Register (r1)
            MOV_REG_REG => {
                let r1 = self.fetch();
                let r2 = self.fetch();

                let r2_value = self.read_register_index(r2 as usize);
                self.write_register_index(r1 as usize, r2_value);
            }

            // Move Register into Memory 
            MOV_REG_MEM => {
                let r1 = self.fetch();
                let address = self.fetch_16();

                let r1_value = self.read_register_index(r1 as usize);
                self.write_dual_bytes(address as usize, r1_value);
            }

            // Move Memory into Register
            MOV_MEM_REG => {
                let address = self.fetch_16();
                let r1 = self.fetch();

                let address_value = self.read_dual_bytes(address as usize);
                self.write_register_index(r1 as usize, address_value);
            }

            // Move Literal into Memory
            MOV_LIT_MEM => {
                let literal = self.fetch_16();
                let address = self.fetch_16();

                self.write_dual_bytes(address as usize, literal);
            }

            // Jump If NOT Equal
            JMP_NOT_EQ => {
                let value = self.fetch_16();
                let address = self.fetch_16();

                if value != self.read_register("rra"){
                    self.write_register("ip", address);
                }
            }

            // Add Register to Register
            ADD_REG_REG => {
                let r1 = self.fetch();
                let r2 = self.fetch();

                let r1_value = self.read_register_index(r1 as usize);
                let r2_value = self.read_register_index(r2 as usize);

                self.write_register("rra", r1_value + r2_value);
            }

            // Add Literal to Register
            ADD_LIT_REG => {
                let literal = self.fetch_16();
                let r1 = self.fetch();

                let r1_value = self.read_register_index(r1 as usize);

                self.write_register("rra", literal + r1_value);
            }

            // Sub Register from Register
            SUB_REG_REG => {
                let r1 = self.fetch();
                let r2 = self.fetch();

                let r1_value = self.read_register_index(r1 as usize);
                let r2_value = self.read_register_index(r2 as usize);

                self.write_register("rra", r2_value - r1_value);
            }

            // Sub Literal from Register
            SUB_LIT_REG => {
                let literal = self.fetch_16();
                let r1 = self.fetch();

                let r1_value = self.read_register_index(r1 as usize);

                self.write_register("rra", r1_value - literal);
            }

            // Sub Register from Literal
            SUB_REG_LIT => {
                let r1 = self.fetch();
                let literal = self.fetch_16();

                let r1_value = self.read_register_index(r1 as usize);

                self.write_register("rra", literal - r1_value);
            }

            // Increment Register
            INC_REG => {
                let r1 = self.fetch();
                let r1_value = self.read_register_index(r1 as usize);

                self.write_register_index(r1 as usize, r1_value + 1);
            }

            // Decrement Register
            DEC_REG => {
                let r1 = self.fetch();
                let r1_value = self.read_register_index(r1 as usize);

                self.write_register_index(r1 as usize, r1_value - 1);
            }

            // Mul Register and Register
            MUL_REG_REG => {
                let r1 = self.fetch();
                let r2 = self.fetch();

                let r1_value = self.read_register_index(r1 as usize);
                let r2_value = self.read_register_index(r2 as usize);

                self.write_register("rra", r1_value * r2_value);
            }

            // Mul Literal and Register
            MUL_LIT_REG => {
                let literal = self.fetch_16();
                let r1 = self.fetch();

                let r1_value = self.read_register_index(r1 as usize);

                self.write_register("rra", literal * r1_value);
            }
            _ => {}
        }
    }

    /*pub fn step(&mut self) {
        let instruction = self.fetch();
        self.execute(instruction);
    }*/

    pub fn debug(&mut self) {
        /*for (index, byte) in self.register_memory.iter().enumerate() {
            println!("Memory[{}]: {:#04X}", index, byte);
        }*/

        for i in 0..self.register_map.len(){
            println!("{} : {:#04X}", REGISTERS[i], bind_u8(self.register_memory[i*2], self.register_memory[i*2+1]));
        }

        print!("\n");
    }

    pub fn run(&mut self) {
        let instruction: u8 = self.fetch();

        if instruction != HLT {
            self.execute(instruction);

            //self.debug();
            //self.view_memory(0x100);

            self.run();
        }
    }

    pub fn view_memory(&mut self, address: u16){
        print!("{:#04X}: ", address);

        for i in 0..7 {
            print!("{:#04X} ", self.program_memory[address as usize + i]);
        }

        print!("\n");
    }
}
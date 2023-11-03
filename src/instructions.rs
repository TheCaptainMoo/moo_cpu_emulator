pub mod instructions {
    // Move Memory
    pub const MOV_LIT_REG: u8 = 0x00; // Move Literal Into Register
    pub const MOV_REG_REG: u8 = 0x01; // Move Register Into Register
    pub const MOV_MEM_REG: u8 = 0x02; // Move Memory Into Register
    pub const MOV_REG_MEM: u8 = 0x03; // Move Register Into Memory
    pub const MOV_LIT_MEM: u8 = 0x04; // Move Literal Into Memory
    // pub const MOV_REG_PTR_REG: u8 = 0x05; // Move Register Pointer into Register
    // pub const MOV_LIT_OFF_REG: u8 = 0x06; // Move Literal + Register (Offset) into Register

    // Conditional / Branching
    pub const JMP_NOT_EQ:  u8 = 0x09; // Jump when not equal to
    
    // Arithmetic 
    pub const ADD_REG_REG: u8 = 0x10; // Add Register to Register
    pub const ADD_LIT_REG: u8 = 0x11; // Add Literal to Register

    pub const SUB_REG_REG: u8 = 0x12; // Sub Register from Register
    pub const SUB_LIT_REG: u8 = 0x13; // Sub Literal from Register
    pub const SUB_REG_LIT: u8 = 0x14; // Sub Register from Literal

    pub const INC_REG:     u8 = 0x15; // Increment Register
    pub const DEC_REG:     u8 = 0x16; // Decrement Register

    pub const MUL_LIT_REG: u8 = 0x17; // Mul (unsigned) Literal and Register
    pub const MUL_REG_REG: u8 = 0x18; // Mul (unsigned) Register and Register

    pub const HLT: u8 = 0x20; // Halt Program
}
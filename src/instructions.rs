pub mod instructions {
    // Move Memory
    pub const MOV_LIT_REG: u8 = 0x00; // Move Literal Into Register
    pub const MOV_REG_REG: u8 = 0x01; // Move Register Into Register
    pub const MOV_MEM_REG: u8 = 0x02; // Move Memory Into Register
    pub const MOV_REG_MEM: u8 = 0x03; // Move Register Into Memory
    pub const MOV_LIT_MEM: u8 = 0x04; // Move Literal Into Memory
    // pub const MOV_REG_PTR_REG: u8 = 0x05; // Move Register Pointer into Register
    // pub const MOV_LIT_OFF_REG: u8 = 0x06; // Move Literal + Register (Offset) into Register
    
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

    // Logical
    pub const LSF_REG_LIT: u8 = 0x19; // Left Shift Register by Literal
    pub const LSF_REG_REG: u8 = 0x1A; // Left Shift Register by Register
    pub const RSF_REG_LIT: u8 = 0x1B; // Right Shift Register by Literal
    pub const RSF_REG_REG: u8 = 0x1C; // Left Shift Register by Register

    pub const AND_REG_LIT: u8 = 0x1D; // Bitwise AND Register by Literal
    pub const AND_REG_REG: u8 = 0x1F; // Bitwise AND Register by Register
    pub const OR_REG_LIT:  u8 = 0x20; // Bitwise OR Register by Literal
    pub const OR_REG_REG:  u8 = 0x21; // Bitwise OR Register by Register
    pub const XOR_REG_LIT: u8 = 0x22; // Bitwise XOR Register by Literal
    pub const XOR_REG_REG: u8 = 0x23; // Bitwise XOR Register by Register
    pub const NOT:         u8 = 0x24; // Bitwise NOT Register

    // Conditional / Branching
    pub const JNE_LIT:     u8 = 0x25; // Jump when Literal NOT Equal
    pub const JNE_REG:     u8 = 0x26; 

    pub const JEQ_REG:     u8 = 0x27; 
    pub const JEQ_LIT:     u8 = 0x28;

    pub const JLT_REG:     u8 = 0x29; 
    pub const JLT_LIT:     u8 = 0x2A; 
    pub const JGT_REG:     u8 = 0x2B; 
    pub const JGT_LIT:     u8 = 0x2C; 

    // System
    pub const HLT:         u8 = 0x30; // Halt Program
}
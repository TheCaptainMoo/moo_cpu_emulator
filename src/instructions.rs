pub mod instructions {
    /*pub const MOV_GRA_LIT: u8 = 0x00;
    pub const MOV_GRB_LIT: u8 = 0x01;
    pub const MOV_GRC_LIT: u8 = 0x02;
    pub const MOV_GRD_LIT: u8 = 0x03;*/

    pub const MOV_LIT_REG: u8 = 0x00; // Move Literal Into Register
    pub const MOV_REG_REG: u8 = 0x01; // Move Register Into Register
    pub const MOV_MEM_REG: u8 = 0x02; // Move Memory Into Register
    pub const MOV_REG_MEM: u8 = 0x03; // Move Register Into Memory
    pub const ADD_REG_REG: u8 = 0x10; 
}
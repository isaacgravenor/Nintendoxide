// Code for the 6502 CPU
pub struct RAM {
    pub data: [u8; 0x2000],
}

pub struct CRegisters {
    pub acm: u8,
    pub rx: u8,
    pub ry: u8,
    pub pc: u16,
    pub sp: u8,
    pub sr: u8,
}

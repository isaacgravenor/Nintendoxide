// Code for the 6502 CPU
pub struct RAM {
    data: [u8; 0x2000],
}

pub struct CRegisters {
    acm: u8,
    rx: u8,
    ry: u8,
    pc: u16,
    sp: u8,
    sr: u8,
}

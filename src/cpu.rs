// Code for the 6502 CPU
pub struct RAM {
    data: [u8; 0x2000],
}
pub struct Registers {
    acm: u8,
    rX: u8,
    rY: u8,
    pc: u16,
    sp: u8,
    sr: u8,
}

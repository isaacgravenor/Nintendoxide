mod ppu;

// Code for the 6502 CPU
pub struct RAM {
    data: [u8; 0x2000],
}

pub struct CRegisters {
    acm: u8,
    rX: u8,
    rY: u8,
    pc: u16,
    sp: u8,
    sr: u8,
}

pub struct MemoryMap {
    pub ram: RAM,
    pub ppu_registers: ppu::PRegisters,
    pub cpu_registers: CRegisters,
}

impl MemoryMap {
    pub fn new(&mut Self) -> Self {
        Self {
            ram: RAM { data: [0; 0x2000] },
            ppu_registers: ppu::PRegisters::new(),
            cpu_registers: CRegisters::new(),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // NES memory map
            0x0000..=0x1FFF => self.ram.data[addr],
            0x2000..=0x3FFF => self.ram.data[addr],
            0x4000..=0x5FFF => self.ram.data[addr],
            0x6000..=0x7FFF => self.ram.data[addr],
            0x8000..=0xFFFF => self.ram.data[addr],
            _ => 0,
        }
    }
}

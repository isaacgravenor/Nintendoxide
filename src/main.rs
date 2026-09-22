mod cpu;
mod ppu;

pub struct MemoryMap {
    pub ram: cpu::RAM,
    pub ppu_registers: ppu::PRegisters,
    pub cpu_registers: cpu::CRegisters,
}

impl MemoryMap {
    pub fn new() -> Self {
        Self {
<<<<<<< HEAD
            ram: cpu::RAM { data: [0; 0x2000] },
            ppu_registers: ppu::PRegisters {
                ppuctrl: 0,
                ppumask: 0,
                ppustatus: 0,
                oamaddr: 0,
                oamdata: 0,
                ppuscroll: 0,
                ppuaddr: 0,
                ppudata: 0,
                oamdma: 0,
            },
            cpu_registers: cpu::CRegisters {
                acm: 0,
                rx: 0,
                ry: 0,
                pc: 0,
                sp: 0,
                sr: 0,
            },
=======
            ram: { data: [0; 0x2000] },
            ppu_registers:  {},
            cpu_registers: cpu::CRegisters {},
>>>>>>> a0a4bcedd3f3c3d92caf4de07dfef38ab7cecdc9
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

fn main() {
    println!("Hello, world!");
}

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
            ram: { data: [0; 0x2000] },
            ppu_registers:  {},
            cpu_registers: cpu::CRegisters {},
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

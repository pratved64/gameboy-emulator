use crate::ppu::PPU;

pub struct MemoryBus {
    memory: [u8; 0x10000],
    ppu: PPU,
}

impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus {
            memory: [0; 0x10000],
            ppu: PPU { vram: [0; 0x2000] },
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.ppu.vram[(address - 0x8000) as usize],
            0xFF44 => 0x90,
            0xFF00..=0xFF7F => {
                println!("Read from 0x{:x}", address);
                0x000000
            }

            _ => self.memory[address as usize],
        }
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        match address {
            0x8000..=0x9FFF => self.ppu.vram[(address - 0x8000) as usize] = byte,
            0xFF00..=0xFF7F => println!("Write to 0x{:x}", address),
            _ => self.memory[address as usize] = byte,
        }
    }

    // little endian, read low byte then high
    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address + 1) as u16;

        (high << 8) | low
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let low = (value & 0x00FF) as u8;
        let high = ((value & 0xFF00) >> 8) as u8;

        self.write_byte(address, low);
        self.write_byte(address + 1, high);
    }
}

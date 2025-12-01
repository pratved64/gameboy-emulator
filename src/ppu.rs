pub struct PPU {
    pub vram: [u8; 0x2000],
}

impl PPU {
    pub fn new() -> Self {
        PPU { vram: [0; 0x2000] }
    }
}

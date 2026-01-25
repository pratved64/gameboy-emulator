pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

pub struct PPU {
    pub vram: [u8; 0x2000],
    pub buffer: [u32; SCREEN_HEIGHT * SCREEN_WIDTH],
    pub ly: u8,
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            vram: [0; 0x2000],
            buffer: [0; SCREEN_HEIGHT * SCREEN_WIDTH],
            ly: 0,
        }
    }

    pub fn tick(&mut self, lcdc: u8, scx: u8, scy: u8) {
        self.ly = self.ly.wrapping_add(1);

        if self.ly == 144 {
            self.render_background(lcdc, scx, scy);
        }

        if self.ly >= 154 {
            self.ly = 0;
        }
        // println!("LCDC = {:#04x}", lcdc);
    }

    fn get_non_white(&self) -> Vec<u8> {
        self.vram.into_iter().filter(|&x| x == 0x00).collect()
    }

    fn render_background(&mut self, lcdc: u8, scx: u8, scy: u8) {
        if (lcdc & 0x80) == 0 {
            return;
        }

        let use_8k = (lcdc & 0x10) != 0;
        let bg_map_base = if (lcdc & 0x08) != 0 { 0x1C00 } else { 0x1800 };

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let map_x = (x as u8).wrapping_add(scx);
                let map_y = (y as u8).wrapping_add(scy);

                let tile_col = (map_x / 8) as usize;
                let tile_row = (map_y / 8) as usize;

                let tile_idx = tile_row * 32 + tile_col;

                let tile_id = self.vram[bg_map_base + tile_idx];

                let tile_addr = if use_8k {
                    tile_id as usize * 16
                } else {
                    let signed_id = tile_id as i8 as i16;
                    (0x1000i16 + signed_id * 16) as usize
                };

                let line = (map_y & 7) as usize;
                // if tile_addr + line * 2 + 1 >= 0x2000 {
                //     continue;
                // }

                let b1 = self.vram[tile_addr + line * 2];
                let b2 = self.vram[tile_addr + line * 2 + 1];

                let bit_idx = 7 - (map_x & 7);
                let low_bit = (b1 >> bit_idx) & 1;
                let high_bit = (b2 >> bit_idx) & 1;
                let color_val = (high_bit << 1) | low_bit;

                let color = match color_val {
                    0 => 0xFFFFFF,
                    1 => 0xAAAAAA,
                    2 => 0x555555,
                    3 => 0x000000,
                    _ => 0x000000,
                };

                self.buffer[y * SCREEN_WIDTH + x] = color;
            }
        }
    }

    fn debug_draw_tiles(&mut self) {
        let mut xdraw = 0;
        let mut ydraw = 0;

        for i in (0..0x1800).step_by(16) {
            self.draw_tile(i, xdraw, ydraw);

            xdraw += 8;
            if xdraw >= SCREEN_WIDTH {
                xdraw = 0;
                ydraw += 8;
            }

            if ydraw >= SCREEN_HEIGHT {
                break;
            }
        }
    }

    fn draw_tile(&mut self, start: usize, x: usize, y: usize) {
        for row in 0..8 {
            let byte1 = self.vram[start + (row * 2)];
            let byte2 = self.vram[start + (row * 2) + 1];

            for col in 0..8 {
                let bit_idx = 7 - col;

                let low_bit = (byte1 >> bit_idx) & 1;
                let high_bit = (byte2 >> bit_idx) & 1;

                let color_val = (high_bit << 1) | low_bit;

                let color = match color_val {
                    0 => 0xFFFFFF,
                    1 => 0xAAAAAA,
                    2 => 0x555555,
                    3 => 0x000000,
                    _ => 0x000000,
                };

                let fx = x + col;
                let fy = y + row;

                if fx < SCREEN_WIDTH && fy < SCREEN_HEIGHT {
                    let pix_idx = fy * SCREEN_WIDTH + fx;
                    self.buffer[pix_idx] = color;
                }
            }
        }
    }
}

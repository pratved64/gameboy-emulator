mod bus;
mod cpu;
mod instruction;
mod ppu;

use bus::MemoryBus;
use cpu::CPU;
use minifb::{Window, WindowOptions};
use ppu::{SCREEN_HEIGHT, SCREEN_WIDTH};
use std::error::Error;
use std::fs;

// Bootrom maybe working!
// TODO: Test the emulator with a gb ROM and see what happens
// Expected: The PC should continue past 0x0100 and not get stuck in this infinite loop!

fn main() -> Result<(), Box<dyn Error>> {
    let mut bus = MemoryBus::new();
    let mut cpu = CPU::new();

    let bootrom: Vec<u8> = fs::read("dmg_boot.bin")?;

    let gamerom = fs::read("Tetris (World) (Rev 1).gb").unwrap_or_else(|_| {
        println!("Warning: could not find gamerom, loading dummy rom!");
        vec![0; 0x8000]
    });

    for (i, byte) in gamerom.iter().enumerate() {
        if i < 0x10000 {
            bus.write_byte(i as u16, *byte);
        }
    }

    for (i, byte) in bootrom.iter().enumerate() {
        bus.write_byte(i as u16, *byte);
    }

    println!(
        "System loaded. BootROM: {} bytes | GameROM: {} bytes",
        bootrom.len(),
        gamerom.len()
    );

    let mut window = Window::new(
        "Gameboy",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    let mut executed_count: u32 = 0;

    println!("FF50 (boot disable) = {:#04x}", bus.read_byte(0xFF50));

    let mut dumped = false;

    let mut scanline_counter = 0;

    while window.is_open() {
        // --- TRACE START ---
        // Kept your trace filter so logs don't explode.
        // This only logs the critical handover from Boot ROM to Tetris.
        let opcode = bus.read_byte(cpu.pc);
        println!(
            "PC: {:#04x} | Op: {:#02x} | SP: {:#04x} | A: {:#02x} | B: {:#02x} | HL: {:#04x}",
            cpu.pc,
            opcode,
            cpu.sp,
            cpu.registers.a,
            cpu.registers.b,
            cpu.registers.get_hl()
        );
        // --- TRACE END ---

        cpu.handle_interrupts(&mut bus);
        cpu.step(&mut bus);
        executed_count += 1;

        scanline_counter += 1;

        if scanline_counter >= 114 {
            scanline_counter = 0;

            let vblank_triggered = bus.ppu.tick(
                bus.read_byte(0xFF40),
                bus.read_byte(0xFF43),
                bus.read_byte(0xFF42),
            );

            if vblank_triggered {
                let mut if_reg = bus.read_byte(0xFF0F);
                if_reg |= 1;
                bus.write_byte(0xFF0F, if_reg);

                window
                    .update_with_buffer(&bus.ppu.buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
                    .unwrap();
            }
        }

        if executed_count > 200_000 && !dumped {
            bus.ppu.dump_vram();
            dumped = true;
        }
    }

    Ok(())
}

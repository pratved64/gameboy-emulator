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

    let mut executed_count = 0;

    //loop {}

    println!("FF50 (boot disable) = {:#04x}", bus.read_byte(0xFF50));

    while window.is_open() {
        cpu.step(&mut bus);
        executed_count += 1;
        bus.ppu.tick(bus.read_byte(0xFF40));
        // if cpu.pc >= 0x0100 {
        //     println!("BootROM execution completed!");
        //     break;
        // }
        if executed_count > 200_000 {
            println!("Exceeded 200,000 steps!");
            println!("PC: {:#06x}", cpu.pc);
            break;
        }

        window
            .update_with_buffer(&bus.ppu.buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }

    Ok(())
}

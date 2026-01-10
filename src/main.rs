mod bus;
mod cpu;
mod instruction;
mod ppu;

use bus::MemoryBus;
use cpu::CPU;
use std::error::Error;
use std::fs;

// Bootrom maybe working!
// TODO: Test the emulator with a gb ROM and see what happens
// Expected: The PC should continue past 0x0100 and not get stuck in this infinite loop!

fn main() -> Result<(), Box<dyn Error>> {
    let mut bus = MemoryBus::new();
    let mut cpu = CPU::new();

    let bootrom: Vec<u8> = fs::read("dmg_boot.bin")?;

    for (i, byte) in bootrom.iter().enumerate() {
        bus.write_byte(i as u16, *byte);
    }

    println!(
        "Bootrom loaded {} bytes. Starting execution...",
        bootrom.len()
    );

    let mut executed_count = 0;

    loop {
        cpu.step(&mut bus);
        executed_count += 1;
        if cpu.pc >= 0x0100 {
            println!("Bootrom execution finished!");
            break;
        }
        if executed_count > 50000 {
            println!("Exceeded 50,000 steps!");
            println!("PC: {:#06x}", cpu.pc);
            break;
        } else if executed_count >= 49000 {
            println!(
                "PC: {:#06x} | A: {:#06x} | Carry: {}",
                cpu.pc, cpu.registers.a, cpu.registers.f.carry
            );
        }
    }

    let mut logo_found = false;
    for i in 0x8004..0x8014 {
        if bus.read_byte(i) != 0 {
            logo_found = true;
            break;
        }
    }

    if logo_found {
        println!("Logo data was found!");
    } else {
        println!("Logo data not found!");
    }

    Ok(())
}

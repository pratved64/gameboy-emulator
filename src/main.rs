mod bus;
mod cpu;
mod instruction;
mod ppu;

use bus::MemoryBus;
use cpu::CPU;
use std::error::Error;
use std::fs;

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

    loop {
        cpu.step(&mut bus);

        if cpu.pc >= 0x0100 {
            println!("Bootrom execution finished!");
            break;
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

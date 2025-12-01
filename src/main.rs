mod bus;
mod cpu;
mod instruction;
mod ppu;

use bus::MemoryBus;
use cpu::CPU;

fn main() {
    let mut bus = MemoryBus::new();
    let mut cpu = CPU::new();

    println!("Test: BIT 7, H");

    // Case 1: H = 0x80 (Bit 7 is Set)
    cpu.registers.h = 0x80;
    // Inject: PREFIX (0xCB) then BIT 7, H (0x7C)
    bus.write_byte(0x00, 0xCB);
    bus.write_byte(0x01, 0x7C);

    cpu.step(&mut bus);
    let z_flag_set = cpu.registers.f.zero;
    println!("H=0x80 | Zero Flag: {} (Expected: false)", z_flag_set);

    // Case 2: H = 0x7F (Bit 7 is Cleared -> 0111 1111)
    cpu.registers.h = 0x7F;
    cpu.pc = 0x00; // Reset PC to run the same instruction again
    // (Memory still contains 0xCB, 0x7C)

    cpu.step(&mut bus);
    let z_flag_clear = cpu.registers.f.zero;
    println!("H=0x7F | Zero Flag: {} (Expected: true)", z_flag_clear);

    if !z_flag_set && z_flag_clear {
        println!("✅ SUCCESS: Prefix CB decoding and BIT logic working.");
    } else {
        println!("❌ FAIL: Check your logic.");
    }

    // debug unknown instruction
}

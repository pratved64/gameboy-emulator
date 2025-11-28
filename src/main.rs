mod bus;
mod cpu;
mod instruction;

use bus::MemoryBus;
use cpu::CPU;

fn main() {
    let mut bus = MemoryBus::new();
    let mut cpu = CPU::new();

    // 1. Initialize SP
    cpu.sp = 0xFFFE;

    // A simple program:
    // 0x00: CALL 0x0006  (Jump to func at index 6)
    // 0x03: 0x02         (Target Address for CALL)
    // 0x04: 0x00         (Padding / Next instruction)
    // 0x06: ADD A, 0x10  (The Function: Add 0x10 to A. Wait, we don't have ADD Immediate yet! Let's use XOR A to clear it)
    // 0x07: RET          (Return)

    // Let's execute:
    // 1. CALL 0x0004
    // 2. XOR A (At 0x0004)
    // 3. RET

    let bootrom: [u8; 6] = [
        0xCD, 0x04, 0x00, // CALL 0x0004 (3 bytes)
        0x00, // NOP (Padding to fill space at 0x03)
        0xAF, // FUNC: XOR A  (Address 0x0004)
        0xC9, // FUNC: RET    (Address 0x0005)
    ];

    for (i, byte) in bootrom.iter().enumerate() {
        bus.write_byte(i as u16, *byte);
    }

    // Preset A to 0x55 so we can see XOR A clear it
    cpu.registers.a = 0x55;

    println!("Starting CALL/RET Test...");

    for _ in 0..3 {
        println!(
            "PC: {:#06x} | SP: {:#06x} | A: {:#04x}",
            cpu.pc, cpu.sp, cpu.registers.a
        );
        cpu.step(&mut bus);
    }

    println!(
        "Final PC: {:#06x} | SP: {:#06x} | A: {:#04x}",
        cpu.pc, cpu.sp, cpu.registers.a
    );
}

mod bus;
mod cpu;

use bus::MemoryBus;
use cpu::CPU;

fn main() {
    let mut bus = MemoryBus::new();
    let mut cpu = CPU::new();

    let bootrom: [u8; 8] = [0x31, 0xFE, 0xFF, 0x01, 0x34, 0x12, 0xC5, 0xD1];

    for (i, byte) in bootrom.iter().enumerate() {
        bus.write_byte(i as u16, *byte);
    }

    println!("System initialized, starting CPU...");

    for _ in 0..3 {
        println!(
            "PC {:#06x} | SP {:#06x} | A: {:#04x} | HL: {:#06x}",
            cpu.pc,
            cpu.sp,
            cpu.registers.a,
            cpu.registers.get_hl()
        );

        cpu.step(&mut bus);
    }

    println!("Final State:");
    println!("PC: {:#06x}", cpu.pc);
    println!("SP: {:#06x}", cpu.sp);
    println!("HL: {:#06x}", cpu.registers.get_hl());
    println!("A:  {:#04x}", cpu.registers.a);
}

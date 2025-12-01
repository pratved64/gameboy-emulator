//  TODO: Implement remaining instructions and start
//        with Program Counter

use crate::bus::MemoryBus;
use crate::instruction::{ArithmeticTarget, Instruction, JumpTest, Load16Target, StackTarget};

pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                f: FlagsRegister::from(0),
                h: 0,
                l: 0,
            },
            pc: 0,
            sp: 0,
        }
    }

    fn execute(&mut self, instruction: Instruction, bus: &mut MemoryBus) {
        match instruction {
            Instruction::ADD(target) => {
                let value = match target {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                };
                let result = self.add(value);
                self.registers.a = result;
            }

            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };

                let jump_addr = bus.read_word(self.pc);
                self.pc = self.pc.wrapping_add(2);

                if jump_condition {
                    self.pc = jump_addr;
                }
            }

            Instruction::XOR(target) => {
                let value = match target {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                };

                let result = self.registers.a ^ value;
                self.registers.a = result;

                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = false;
            }

            Instruction::LD16(target) => {
                let value = bus.read_word(self.pc);
                self.pc = self.pc.wrapping_add(2);

                match target {
                    Load16Target::BC => self.registers.set_bc(value),
                    Load16Target::DE => self.registers.set_de(value),
                    Load16Target::HL => self.registers.set_hl(value),
                    Load16Target::SP => self.sp = value,
                }
            }

            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::BC => self.registers.get_bc(),
                    StackTarget::DE => self.registers.get_de(),
                    StackTarget::HL => self.registers.get_hl(),
                    StackTarget::AF => self.registers.get_af(),
                };

                self.push(bus, value);
            }

            Instruction::POP(target) => {
                let result = self.pop(bus);
                match target {
                    StackTarget::BC => self.registers.set_bc(result),
                    StackTarget::DE => self.registers.set_de(result),
                    StackTarget::HL => self.registers.set_hl(result),
                    StackTarget::AF => self.registers.set_af(result),
                }
            }

            Instruction::CALL(test) => {
                let target_addr = bus.read_word(self.pc);
                self.pc = self.pc.wrapping_add(2);
                if match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                } {
                    self.push(bus, self.pc);
                    self.pc = target_addr;
                }
            }

            Instruction::RET(test) => {
                if match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                } {
                    let top = self.pop(bus);
                    self.pc = top;
                }
            }

            Instruction::LD_HL_DEC_A => {
                let address = self.registers.get_hl();
                let value = self.registers.a;
                bus.write_byte(address, value);

                self.registers.set_hl(address.wrapping_sub(1));
            }

            Instruction::JR(test) => {
                let offset = bus.read_byte(self.pc) as i8;
                self.pc = self.pc.wrapping_add(1);
                if match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                } {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }

            Instruction::INC(target) => {
                let value = self.read_reg(&target);

                let new_value = value.wrapping_add(1);

                println!(
                    "INC {:?} | Old: {} | New: {} | ZeroFlag: {}",
                    target,
                    value,
                    new_value,
                    new_value == 0,
                );
                self.registers.f.zero = new_value == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = (value & 0xF) == 0xF;

                self.write_reg(target, new_value);
            }

            Instruction::DEC(target) => {
                let value = self.read_reg(&target);
                let new_value = value.wrapping_sub(1);
                self.registers.f.zero = new_value == 0;
                self.registers.f.subtract = true;
                self.registers.f.half_carry = (value & 0xF) == 0;
                self.write_reg(target, new_value);
            }

            Instruction::INC16(target) => match target {
                Load16Target::BC => self
                    .registers
                    .set_bc(self.registers.get_bc().wrapping_add(1)),
                Load16Target::DE => self
                    .registers
                    .set_de(self.registers.get_de().wrapping_add(1)),
                Load16Target::HL => self
                    .registers
                    .set_hl(self.registers.get_hl().wrapping_add(1)),
                Load16Target::SP => self.sp = self.sp.wrapping_add(1),
            },

            Instruction::DEC16(target) => match target {
                Load16Target::BC => self
                    .registers
                    .set_bc(self.registers.get_bc().wrapping_sub(1)),
                Load16Target::DE => self
                    .registers
                    .set_de(self.registers.get_de().wrapping_sub(1)),
                Load16Target::HL => self
                    .registers
                    .set_hl(self.registers.get_hl().wrapping_sub(1)),
                Load16Target::SP => self.sp = self.sp.wrapping_sub(1),
            },

            Instruction::BIT(target) => {
                let value = match target {
                    ArithmeticTarget::H => self.registers.h,
                    _ => panic!("BIT target not implemented!"),
                };

                let result = value & 0x80;
                self.registers.f.zero = result == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = true;
            }

            _ => {} // TODO: Support more instructions
        }
    }

    fn read_reg(&self, target: &ArithmeticTarget) -> u8 {
        match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
        }
    }

    fn write_reg(&mut self, target: ArithmeticTarget, value: u8) {
        match target {
            ArithmeticTarget::A => self.registers.a = value,
            ArithmeticTarget::B => self.registers.b = value,
            ArithmeticTarget::C => self.registers.c = value,
            ArithmeticTarget::D => self.registers.d = value,
            ArithmeticTarget::E => self.registers.e = value,
            ArithmeticTarget::H => self.registers.h = value,
            ArithmeticTarget::L => self.registers.l = value,
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    fn push(&mut self, bus: &mut MemoryBus, value: u16) {
        self.sp = self.sp.wrapping_sub(2);
        bus.write_word(self.sp, value);
    }

    fn pop(&mut self, bus: &mut MemoryBus) -> u16 {
        let top = bus.read_word(self.sp);
        self.sp = self.sp.wrapping_add(2);
        top
    }

    pub fn step(&mut self, bus: &mut MemoryBus) {
        let instruction_byte = bus.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);

        let instruction = Instruction::from_byte(instruction_byte);

        match instruction {
            Some(Instruction::PREFIX) => {
                let cb_byte = bus.read_byte(self.pc);
                self.pc = self.pc.wrapping_add(1);

                if let Some(cb_inst) = Instruction::from_cb_byte(cb_byte) {
                    self.execute(cb_inst, bus);
                } else {
                    panic!("Unknown CB Instruction: 0xCB{:02x}", cb_byte);
                }
            }
            Some(instr) => self.execute(instr, bus),
            None => panic!("Unknown instruction"),
        }
    }
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn get_af(&self) -> u16 {
        let f: u8 = self.f.into();
        (self.a as u16) << 8 | f as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0xF0) as u8);
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_bc(&self) -> u16 {
        // allow reading 16 bytes from registers B and C
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        // allow setting 16 bytes in registers B and C
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
} // special flags register (f)

const ZERO_FLAG_BYTE: u8 = 7;
const SUBTRACT_FLAG_BYTE: u8 = 6;
const HALF_CARRY_FLAG_BYTE: u8 = 5;
const CARRY_FLAG_BYTE: u8 = 4;

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        // convert to u8
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        // convert to FlagsRegister struct
        let zero = ((byte >> ZERO_FLAG_BYTE) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

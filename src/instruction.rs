pub enum Instruction {
    ADD(ArithmeticTarget),
    SUB(ArithmeticTarget),
    CP(ArithmeticTarget),
    XOR(ArithmeticTarget),
    INC(ArithmeticTarget),
    DEC(ArithmeticTarget),
    INC16(Load16Target),
    DEC16(Load16Target),
    JP(JumpTest),
    JR(JumpTest),
    LD(ArithmeticTarget, ArithmeticTarget),
    LD16(Load16Target),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),
    LD_HL_DEC_A,
    BIT(ArithmeticTarget),
    PREFIX,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Load16Target {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StackTarget {
    BC,
    DE,
    HL,
    AF,
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            0x01 => Some(Instruction::LD16(Load16Target::BC)),
            0x11 => Some(Instruction::LD16(Load16Target::DE)),
            0x21 => Some(Instruction::LD16(Load16Target::HL)),
            0x31 => Some(Instruction::LD16(Load16Target::SP)),

            0x32 => Some(Instruction::LD_HL_DEC_A),

            0x18 => Some(Instruction::JR(JumpTest::Always)),

            0x03 => Some(Instruction::INC16(Load16Target::BC)),
            0x13 => Some(Instruction::INC16(Load16Target::DE)),
            0x23 => Some(Instruction::INC16(Load16Target::HL)),
            0x33 => Some(Instruction::INC16(Load16Target::SP)),

            0x0B => Some(Instruction::DEC16(Load16Target::BC)),
            0x1B => Some(Instruction::DEC16(Load16Target::DE)),
            0x2B => Some(Instruction::DEC16(Load16Target::HL)),
            0x3B => Some(Instruction::DEC16(Load16Target::SP)),

            0x40 => Some(Instruction::LD(ArithmeticTarget::B, ArithmeticTarget::B)),
            0x41 => Some(Instruction::LD(ArithmeticTarget::B, ArithmeticTarget::C)),
            0x42 => Some(Instruction::LD(ArithmeticTarget::B, ArithmeticTarget::D)),
            0x43 => Some(Instruction::LD(ArithmeticTarget::B, ArithmeticTarget::E)),
            0x44 => Some(Instruction::LD(ArithmeticTarget::B, ArithmeticTarget::H)),
            0x45 => Some(Instruction::LD(ArithmeticTarget::B, ArithmeticTarget::L)),
            // 0x46 => Some(Instruction::LD(ArithmeticTarget::B, ArithmeticTarget::HL)),
            0x47 => Some(Instruction::LD(ArithmeticTarget::B, ArithmeticTarget::A)),

            0x48 => Some(Instruction::LD(ArithmeticTarget::C, ArithmeticTarget::B)),
            0x49 => Some(Instruction::LD(ArithmeticTarget::C, ArithmeticTarget::C)),
            0x4A => Some(Instruction::LD(ArithmeticTarget::C, ArithmeticTarget::D)),
            0x4B => Some(Instruction::LD(ArithmeticTarget::C, ArithmeticTarget::E)),
            0x4C => Some(Instruction::LD(ArithmeticTarget::C, ArithmeticTarget::H)),
            0x4D => Some(Instruction::LD(ArithmeticTarget::C, ArithmeticTarget::L)),
            // 0x4E => Some(Instruction::LD(ArithmeticTarget::C, ArithmeticTarget::HL)),
            0x4F => Some(Instruction::LD(ArithmeticTarget::C, ArithmeticTarget::A)),

            0x50 => Some(Instruction::LD(ArithmeticTarget::D, ArithmeticTarget::B)),
            0x51 => Some(Instruction::LD(ArithmeticTarget::D, ArithmeticTarget::C)),
            0x52 => Some(Instruction::LD(ArithmeticTarget::D, ArithmeticTarget::D)),
            0x53 => Some(Instruction::LD(ArithmeticTarget::D, ArithmeticTarget::E)),
            0x54 => Some(Instruction::LD(ArithmeticTarget::D, ArithmeticTarget::H)),
            0x55 => Some(Instruction::LD(ArithmeticTarget::D, ArithmeticTarget::L)),
            // 0x56 => Some(Instruction::LD(ArithmeticTarget::D, ArithmeticTarget::HL)),
            0x57 => Some(Instruction::LD(ArithmeticTarget::D, ArithmeticTarget::A)),

            0x58 => Some(Instruction::LD(ArithmeticTarget::E, ArithmeticTarget::B)),
            0x59 => Some(Instruction::LD(ArithmeticTarget::E, ArithmeticTarget::C)),
            0x5A => Some(Instruction::LD(ArithmeticTarget::E, ArithmeticTarget::D)),
            0x5B => Some(Instruction::LD(ArithmeticTarget::E, ArithmeticTarget::E)),
            0x5C => Some(Instruction::LD(ArithmeticTarget::E, ArithmeticTarget::H)),
            0x5D => Some(Instruction::LD(ArithmeticTarget::E, ArithmeticTarget::L)),
            // 0x5E => Some(Instruction::LD(ArithmeticTarget::E, ArithmeticTarget::HL)),
            0x5F => Some(Instruction::LD(ArithmeticTarget::E, ArithmeticTarget::A)),

            0x60 => Some(Instruction::LD(ArithmeticTarget::H, ArithmeticTarget::B)),
            0x61 => Some(Instruction::LD(ArithmeticTarget::H, ArithmeticTarget::C)),
            0x62 => Some(Instruction::LD(ArithmeticTarget::H, ArithmeticTarget::D)),
            0x63 => Some(Instruction::LD(ArithmeticTarget::H, ArithmeticTarget::E)),
            0x64 => Some(Instruction::LD(ArithmeticTarget::H, ArithmeticTarget::H)),
            0x65 => Some(Instruction::LD(ArithmeticTarget::H, ArithmeticTarget::L)),
            // 0x66 => Some(Instruction::LD(ArithmeticTarget::H, ArithmeticTarget::HL)),
            0x67 => Some(Instruction::LD(ArithmeticTarget::H, ArithmeticTarget::A)),

            0x68 => Some(Instruction::LD(ArithmeticTarget::L, ArithmeticTarget::B)),
            0x69 => Some(Instruction::LD(ArithmeticTarget::L, ArithmeticTarget::C)),
            0x6A => Some(Instruction::LD(ArithmeticTarget::L, ArithmeticTarget::D)),
            0x6B => Some(Instruction::LD(ArithmeticTarget::L, ArithmeticTarget::E)),
            0x6C => Some(Instruction::LD(ArithmeticTarget::L, ArithmeticTarget::H)),
            0x6D => Some(Instruction::LD(ArithmeticTarget::L, ArithmeticTarget::L)),
            // 0x6E => Some(Instruction::LD(ArithmeticTarget::L, ArithmeticTarget::HL)),
            0x6F => Some(Instruction::LD(ArithmeticTarget::L, ArithmeticTarget::A)),

            0x3C => Some(Instruction::INC(ArithmeticTarget::A)),
            0x04 => Some(Instruction::INC(ArithmeticTarget::B)),
            0x0C => Some(Instruction::INC(ArithmeticTarget::C)),
            0x14 => Some(Instruction::INC(ArithmeticTarget::D)),
            0x1C => Some(Instruction::INC(ArithmeticTarget::E)),
            0x24 => Some(Instruction::INC(ArithmeticTarget::H)),
            0x2C => Some(Instruction::INC(ArithmeticTarget::L)),

            0x3D => Some(Instruction::DEC(ArithmeticTarget::A)),
            0x05 => Some(Instruction::DEC(ArithmeticTarget::B)),
            0x0D => Some(Instruction::DEC(ArithmeticTarget::C)),
            0x15 => Some(Instruction::DEC(ArithmeticTarget::D)),
            0x1D => Some(Instruction::DEC(ArithmeticTarget::E)),
            0x25 => Some(Instruction::DEC(ArithmeticTarget::H)),
            0x2D => Some(Instruction::DEC(ArithmeticTarget::L)),

            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            // 0x86 is ADD A, (HL)
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),

            0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),
            // 0x96 is SUB A, (HL)
            0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),

            0xB8 => Some(Instruction::CP(ArithmeticTarget::B)),
            0xB9 => Some(Instruction::CP(ArithmeticTarget::C)),
            0xBA => Some(Instruction::CP(ArithmeticTarget::D)),
            0xBB => Some(Instruction::CP(ArithmeticTarget::E)),
            0xBC => Some(Instruction::CP(ArithmeticTarget::H)),
            0xBD => Some(Instruction::CP(ArithmeticTarget::L)),
            // 0xBE is CP A, (HL)
            0xBF => Some(Instruction::CP(ArithmeticTarget::A)),

            0xC3 => Some(Instruction::JP(JumpTest::Always)),
            0xC2 => Some(Instruction::JP(JumpTest::NotZero)),
            0xCA => Some(Instruction::JP(JumpTest::Zero)),

            0xA8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xA9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xAA => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xAB => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xAC => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xAD => Some(Instruction::XOR(ArithmeticTarget::L)),
            0xAF => Some(Instruction::XOR(ArithmeticTarget::A)),

            0xC1 => Some(Instruction::POP(StackTarget::BC)),
            0xD1 => Some(Instruction::POP(StackTarget::DE)),
            0xE1 => Some(Instruction::POP(StackTarget::HL)),
            0xF1 => Some(Instruction::POP(StackTarget::AF)),

            0xC5 => Some(Instruction::PUSH(StackTarget::BC)),
            0xD5 => Some(Instruction::PUSH(StackTarget::DE)),
            0xE5 => Some(Instruction::PUSH(StackTarget::HL)),
            0xF5 => Some(Instruction::PUSH(StackTarget::AF)),

            0xCD => Some(Instruction::CALL(JumpTest::Always)),
            0xC4 => Some(Instruction::CALL(JumpTest::NotZero)),
            0xCC => Some(Instruction::CALL(JumpTest::Zero)),
            0xDC => Some(Instruction::CALL(JumpTest::NotCarry)),
            0xD4 => Some(Instruction::CALL(JumpTest::Carry)),

            0xC9 => Some(Instruction::RET(JumpTest::Always)),
            0xC0 => Some(Instruction::RET(JumpTest::NotZero)),
            0xC8 => Some(Instruction::RET(JumpTest::Zero)),
            0xD0 => Some(Instruction::RET(JumpTest::NotCarry)),
            0xD8 => Some(Instruction::RET(JumpTest::Carry)),
            _ => None,
        }
    }

    pub fn from_cb_byte(byte: u8) -> Option<Instruction> {
        match byte {
            0x7C => Some(Instruction::BIT(ArithmeticTarget::H)),
            _ => None,
        }
    }
}

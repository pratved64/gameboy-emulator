pub enum Instruction {
    ADD(ArithmeticTarget),
    XOR(ArithmeticTarget),
    INC(ArithmeticTarget),
    DEC(ArithmeticTarget),
    INC16(Load16Target),
    DEC16(Load16Target),
    JP(JumpTest),
    JR(JumpTest),
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

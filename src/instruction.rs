pub enum Instruction {
    ADD(ArithmeticTarget),
    XOR(ArithmeticTarget),
    JP(JumpTest),
    LD16(Load16Target),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

pub enum Load16Target {
    BC,
    DE,
    HL,
    SP,
}

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
}

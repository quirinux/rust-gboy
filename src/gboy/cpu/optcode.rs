use super::*;
use super::super::*;

#[derive(Debug, Copy, Clone)]
pub enum OptCode {

    NOP,
    LDnn(RegisterType, u8, u8),
    XORn(RegisterType),
    LDDHLA,
    JRCCn(FlagRegisterType, u8, i8),
    LDNNn(RegisterType, u8),
    LDNn(RegisterType, RegisterType),
    LDnnA(u8, u8),
    LDNnAddress(RegisterType, RegisterType),
    LDCA,
    INCn(RegisterType),
    INCnn(RegisterType),
    LDnA(RegisterType),
    LDHnA(u8),
    LDHAn(u8),
    Call(u8, u8),
    PushNN(RegisterType),
    PopNN(RegisterType),
    RET,
    RLn(RegisterType),
    DecN(RegisterType),
    LDIHLA,
    SUBn(RegisterType),
    CPn(RegisterType),
    CPnAddress(RegisterType),
    CPnValue(u8),
    JRn(i8),
    JPnn(u8, u8),
    ADDnn(RegisterType, RegisterType),
    DI,
    EI,
    
    None(u8),

    // CBs
    CBBit7H,
    CBRLn(RegisterType),

    CBNone(u8),
}

impl Default for OptCode {
    fn default() -> OptCode {
        OptCode::NOP
    }
}

impl Cpu {
    pub fn decode(&mut self, optcode: u8) -> OptCode {
        match optcode {
            0xCB => {
                let cb_optcode = self.read_instruction();
                self.cb_decode(cb_optcode)
            },

            // Nop
            0x0 => OptCode::NOP,
            
            // LD (BC | DE | HL | SP),nn
            // 16bits instructions are stored in Bigendian instead of Littleendian
            // turning debuging process breezier
            0x01 | 0x11 | 0x21 | 0x31 => {
                let a = self.read_instruction();
                let b = self.read_instruction();
                match optcode {
                    0x01 => OptCode::LDnn(RegisterType::BC, b, a),
                    0x11 => OptCode::LDnn(RegisterType::DE, b, a),
                    0x21 => OptCode::LDnn(RegisterType::HL, b, a),
                    0x31 => OptCode::LDnn(RegisterType::SP, b, a),
                    _ => panic!("LD 16b, nn not found => {:#x}", optcode),
                }
            },

            // Inc NN
            0x03 => OptCode::INCnn(RegisterType::BC),
            0x13 => OptCode::INCnn(RegisterType::DE),
            0x23 => OptCode::INCnn(RegisterType::HL),
            0x33 => OptCode::INCnn(RegisterType::SP),
            

            
            // XOR A,A
            0xAF => OptCode::XORn(RegisterType::A),
            // XOR B,A
            0xA8 => OptCode::XORn(RegisterType::B),
            // XOR C,A
            0xA9 => OptCode::XORn(RegisterType::C),
            // XOR D,A
            0xAA => OptCode::XORn(RegisterType::D),
            // XOR D,A
            0xAB => OptCode::XORn(RegisterType::E),
            // XOR H,A
            0xAC => OptCode::XORn(RegisterType::H),
            // XOR L,A
            0xAD => OptCode::XORn(RegisterType::L),

            // LD (HLD),A | LD (HL-),A | LDD (HL),A 
            0x32 => OptCode::LDDHLA,

            // JR cc,n
            0x20 | 0x28 | 0x30 | 0x38 => {
                let a: i8 = self.read_instruction() as i8;
                match optcode {
                    0x20 => OptCode::JRCCn(FlagRegisterType::Zero, 0, a), // Jump if Z flag is reset
                    0x28 => OptCode::JRCCn(FlagRegisterType::Zero, 1, a), // Jump if Z flag is set
                    0x30 => OptCode::JRCCn(FlagRegisterType::Carry, 0, a), // Jump if C flag is reset
                    0x38 => OptCode::JRCCn(FlagRegisterType::Carry, 1, a), // Jump if C flag is set
                    _ => panic!("JR cc,n not found => {:#x}", optcode),
                }
            },

            // LD nn,n
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E => {
                let a = self.read_instruction();
                match optcode {
                    0x06 => OptCode::LDNNn(RegisterType::B, a),
                    0x0E => OptCode::LDNNn(RegisterType::C, a),
                    0x16 => OptCode::LDNNn(RegisterType::D, a),
                    0x1E => OptCode::LDNNn(RegisterType::E, a),
                    0x26 => OptCode::LDNNn(RegisterType::H, a),
                    0x2E => OptCode::LDNNn(RegisterType::L, a),
                    _ => panic!("LD nn,n not found => {:#x}", optcode),
                }
            },

            // LD a,n
            0x7F | 0x78 | 0x79 | 0x7A | 0x7B | 0x7C | 0x7D | 0x0A | 0x1A | 0x7E | 0xFA | 0x3E => {
                match optcode {
                    0x7F => OptCode::LDNn(RegisterType::A, RegisterType::A),
                    0x78 => OptCode::LDNn(RegisterType::B, RegisterType::A),
                    0x79 => OptCode::LDNn(RegisterType::B, RegisterType::A),
                    0x7A => OptCode::LDNn(RegisterType::D, RegisterType::A),
                    0x7B => OptCode::LDNn(RegisterType::E, RegisterType::A),
                    0x7C => OptCode::LDNn(RegisterType::H, RegisterType::A),
                    0x7D => OptCode::LDNn(RegisterType::L, RegisterType::A),
                    
                    0x0A => OptCode::LDNnAddress(RegisterType::BC, RegisterType::A),
                    0x1A => OptCode::LDNnAddress(RegisterType::DE, RegisterType::A),
                    0x7E => OptCode::LDNnAddress(RegisterType::HL, RegisterType::A),
                    0x3E => {
                        let v = self.read_instruction();
                        OptCode::LDNNn(RegisterType::A, v)
                    },
                    _ => panic!("LD a,n not found => {:#x}", optcode),
                }
            }
            0xE2 => OptCode::LDCA,

            // LD n,A
            0x47 | 0x4f | 0x57 | 0x5F | 0x67 | 0x6F | 0x02 | 0x12 | 0x77 | 0xEA => {
                match optcode  {
                    0x47 => OptCode::LDNn(RegisterType::A, RegisterType::B),
                    0x4f => OptCode::LDNn(RegisterType::A, RegisterType::C),
                    0x57 => OptCode::LDNn(RegisterType::A, RegisterType::D),
                    0x5F => OptCode::LDNn(RegisterType::A, RegisterType::E),
                    0x67 => OptCode::LDNn(RegisterType::A, RegisterType::H),
                    0x6F => OptCode::LDNn(RegisterType::A, RegisterType::L),

                    0x02 => OptCode::LDnA(RegisterType::BC),
                    0x12 => OptCode::LDnA(RegisterType::DE),
                    0x77 => OptCode::LDnA(RegisterType::HL),
                    0xEA => {
                        let a = self.read_instruction();
                        let b = self.read_instruction();
                        OptCode::LDnnA(b, a)
                    },
                    _ => panic!("LD n,a not found => {:#x}", optcode),
                }
            }
            
            // INC n
            0x3C => OptCode::INCn(RegisterType::A),
            0x04 => OptCode::INCn(RegisterType::B),
            0x0C => OptCode::INCn(RegisterType::C),
            0x14 => OptCode::INCn(RegisterType::D),
            0x1C => OptCode::INCn(RegisterType::E),
            0x24 => OptCode::INCn(RegisterType::H),
            0x2C => OptCode::INCn(RegisterType::L),

            // LDH n,A
            0xE0 => {
                let a = self.read_instruction();
                OptCode::LDHnA(a)
            },

            // LDH A,n
            0xF0 => {
                let a = self.read_instruction();
                OptCode::LDHAn(a)
            },
            
            // Call nn
            0xCD => {
                let a = self.read_instruction();
                let b = self.read_instruction();
                OptCode::Call(b, a)
            }

            // Push nn
            0xF5 => OptCode::PushNN(RegisterType::AF),
            0xC5 => OptCode::PushNN(RegisterType::BC),
            0xD5 => OptCode::PushNN(RegisterType::DE),
            0xE5 => OptCode::PushNN(RegisterType::HL),

            // Pop NN
            0xF1 => OptCode::PopNN(RegisterType::AF),
            0xC1 => OptCode::PopNN(RegisterType::BC),
            0xD1 => OptCode::PopNN(RegisterType::DE),
            0xE1 => OptCode::PopNN(RegisterType::HL),
            
            // RLA
            0x17 => OptCode::RLn(RegisterType::A),

            // Dec N
            0x3D => OptCode::DecN(RegisterType::A),
            0x05 => OptCode::DecN(RegisterType::B),
            0x0D => OptCode::DecN(RegisterType::C),
            0x15 => OptCode::DecN(RegisterType::D),
            0x1D => OptCode::DecN(RegisterType::E),
            0x25 => OptCode::DecN(RegisterType::H),
            0x2D => OptCode::DecN(RegisterType::L),

            // LDI (HL),A
            0x22 => OptCode::LDIHLA,

            // Ret
            0xC9 => OptCode::RET,

            // CP n
            0xBF => OptCode::CPn(RegisterType::A),
            0xB8 => OptCode::CPn(RegisterType::B),
            0xB9 => OptCode::CPn(RegisterType::C),
            0xBA => OptCode::CPn(RegisterType::D),
            0xBB => OptCode::CPn(RegisterType::E),
            0xBC => OptCode::CPn(RegisterType::H),
            0xBD => OptCode::CPn(RegisterType::L),
            0xBE => OptCode::CPnAddress(RegisterType::HL),
            0xFE => {
                let a = self.read_instruction();
                OptCode::CPnValue(a)
            },

            // Sub n
            0x97 => OptCode::SUBn(RegisterType::A),
            0x90 => OptCode::SUBn(RegisterType::B),
            0x91 => OptCode::SUBn(RegisterType::C),
            0x92 => OptCode::SUBn(RegisterType::D),
            0x93 => OptCode::SUBn(RegisterType::E),
            0x94 => OptCode::SUBn(RegisterType::H),
            0x95 => OptCode::SUBn(RegisterType::L),

            // JR n
            0x18 => {
                let a: i8 = self.read_instruction() as i8;
                OptCode::JRn(a)
            },

            // JN nn
            0xC3 => {
                let a = self.read_instruction();
                let b = self.read_instruction();
                OptCode::JPnn(b, a)
            },

            // Add n, n
            0x87 => OptCode::ADDnn(RegisterType::A, RegisterType::A),
            0x80 => OptCode::ADDnn(RegisterType::A, RegisterType::B),
            0x81 => OptCode::ADDnn(RegisterType::A, RegisterType::C),
            0x82 => OptCode::ADDnn(RegisterType::A, RegisterType::D),
            0x83 => OptCode::ADDnn(RegisterType::A, RegisterType::E),
            0x84 => OptCode::ADDnn(RegisterType::A, RegisterType::H),
            0x85 => OptCode::ADDnn(RegisterType::A, RegisterType::L),
            0x86 => OptCode::ADDnn(RegisterType::A, RegisterType::HL),

            // DI
            0xF3 => OptCode::DI,
            // EI
            0xFB => OptCode::EI,

            // Instruction not found
            _ => OptCode::None(optcode),
        }
    }

    fn cb_decode(&mut self, cb_optcode: u8) -> OptCode {
        match cb_optcode {
            0x7C => OptCode::CBBit7H,

            // RL n
            0x17 => OptCode::CBRLn(RegisterType::A),
            0x10 => OptCode::CBRLn(RegisterType::B),
            0x11 => OptCode::CBRLn(RegisterType::C),
            0x12 => OptCode::CBRLn(RegisterType::D),
            0x13 => OptCode::CBRLn(RegisterType::E),
            0x14 => OptCode::CBRLn(RegisterType::H),
            0x15 => OptCode::CBRLn(RegisterType::L),
            
            _ => OptCode::CBNone(cb_optcode),
        }
    }
}

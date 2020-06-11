
use super::*;
use crate::gboy::cpu::optcode::*;
use crate::gboy::cpu::registers::*;

impl Cpu {

    pub(crate) fn timer_tick(&mut self, optcode: &OptCode) {
        self.clock_timing(&optcode);
        self.div();
        self.tima();
    }


    // internals
    fn tima(&mut self) {
        
        
        trace!("timer => tma");
    }

    fn div(&mut self) {
        let div_frequency = 256; // TODO: needs change to dynamic cpu frequency
        let cc_elapsed = self.registers.clock_cycles() - self.controls.div_control;
        trace!("div => div_control:{} cc_elpased:{}", self.controls.div_control, cc_elapsed);
        if cc_elapsed >= div_frequency {
            self.memory[FF00 + 04] = self.memory[FF00 + 04].wrapping_add(1);
            self.controls.div_control = self.registers.clock_cycles();
        }
    }
    
    fn clock_timing(&mut self, optcode: &OptCode) {
        let clock_cycles = match *optcode {
            // 16 cycles block
            OptCode::LDnnA(_, _) |
            OptCode::PushNN(_)
                => 16,
            
            // 12 cycles block
            OptCode::Call(_, _) |
            OptCode::JPnn(_, _) |
            OptCode::LDHnA(_) |
            OptCode::LDHAn(_) |
            OptCode::INCn(RegisterType::SP) |
            OptCode::LDDHLA |
            OptCode::LDnn(RegisterType::BC , _, _) |
            OptCode::LDnn(RegisterType::DE , _, _) |
            OptCode::LDnn(RegisterType::HL , _, _) |
            OptCode::LDnn(RegisterType::SP , _, _)
                => 12,

            // 8 cycles block
            OptCode::JRn(_) |
            OptCode::PopNN(RegisterType::AF) |
            OptCode::PopNN(RegisterType::BC) |
            OptCode::PopNN(RegisterType::DE) |
            OptCode::PopNN(RegisterType::HL) |
            OptCode::CBRLn(RegisterType::A) |
            OptCode::CBRLn(RegisterType::B) |
            OptCode::CBRLn(RegisterType::C) |
            OptCode::CBRLn(RegisterType::D) |
            OptCode::CBRLn(RegisterType::E) |
            OptCode::CBRLn(RegisterType::H) |
            OptCode::CBRLn(RegisterType::L) |            
            OptCode::LDNnAddress(RegisterType::DE, _) |
            OptCode::LDnA(RegisterType::BC) |
            OptCode::LDnA(RegisterType::DE) |
            OptCode::LDnA(RegisterType::HL) |
            OptCode::JRCCn(FlagRegisterType::Zero, _, _) |
            OptCode::JRCCn(FlagRegisterType::Carry, _, _) |
            OptCode::LDNNn(RegisterType::A, _) |
            OptCode::LDNNn(RegisterType::B, _) |
            OptCode::LDNNn(RegisterType::C, _) |
            OptCode::LDNNn(RegisterType::D, _) |
            OptCode::LDNNn(RegisterType::E, _) |
            OptCode::LDNNn(RegisterType::H, _) |
            OptCode::LDNNn(RegisterType::L, _) |
            OptCode::LDCA |
            OptCode::LDIHLA |
            OptCode::INCnn(RegisterType::BC) |
            OptCode::INCnn(RegisterType::DE) |
            OptCode::INCnn(RegisterType::HL) |
            OptCode::INCnn(RegisterType::SP) |
            OptCode::CBBit7H |
            OptCode::CPnValue(_) |
            OptCode::CPnAddress(_) |
            OptCode::ADDnn(RegisterType::A, RegisterType::HL) |
            OptCode::RET
                => 8,
            
            
            // 4 cycles block
            OptCode::DI |
            OptCode::EI |
            OptCode::NOP |
            OptCode::SUBn(RegisterType::A) |
            OptCode::SUBn(RegisterType::B) |
            OptCode::SUBn(RegisterType::C) |
            OptCode::SUBn(RegisterType::D) |
            OptCode::SUBn(RegisterType::E) |
            OptCode::SUBn(RegisterType::H) |
            OptCode::SUBn(RegisterType::L) |
            OptCode::LDNn(RegisterType::A, RegisterType::A) |
            OptCode::LDNn(RegisterType::B, RegisterType::A) |
            OptCode::LDNn(RegisterType::C, RegisterType::A) |
            OptCode::LDNn(RegisterType::D, RegisterType::A) |
            OptCode::LDNn(RegisterType::E, RegisterType::A) |
            OptCode::LDNn(RegisterType::H, RegisterType::A) |
            OptCode::LDNn(RegisterType::L, RegisterType::A) |
            OptCode::DecN(RegisterType::A) |
            OptCode::DecN(RegisterType::B) |
            OptCode::DecN(RegisterType::C) |
            OptCode::DecN(RegisterType::D) |
            OptCode::DecN(RegisterType::E) |
            OptCode::DecN(RegisterType::H) |
            OptCode::DecN(RegisterType::L) |
            OptCode::RLn(RegisterType::A) |
            OptCode::LDNn(RegisterType::A, RegisterType::A) |
            OptCode::LDNn(RegisterType::A, RegisterType::B) |
            OptCode::LDNn(RegisterType::A, RegisterType::C) |
            OptCode::LDNn(RegisterType::A, RegisterType::D) |
            OptCode::LDNn(RegisterType::A, RegisterType::E) |
            OptCode::LDNn(RegisterType::A, RegisterType::H) |
            OptCode::LDNn(RegisterType::A, RegisterType::L) |
            OptCode::INCn(RegisterType::A) |
            OptCode::INCn(RegisterType::B) |
            OptCode::INCn(RegisterType::C) |
            OptCode::INCn(RegisterType::D) |
            OptCode::INCn(RegisterType::E) |
            OptCode::INCn(RegisterType::H) |
            OptCode::INCn(RegisterType::L) |
            OptCode::XORn(RegisterType::A) |
            OptCode::XORn(RegisterType::B) |
            OptCode::XORn(RegisterType::C) |
            OptCode::XORn(RegisterType::D) |
            OptCode::XORn(RegisterType::E) |
            OptCode::XORn(RegisterType::H) |
            OptCode::XORn(RegisterType::L)
                => 4,
            _ => {
                error!("optcode clock cycles not defined for => {:?}", optcode);
                self.controls.quit = true;
                0
            }
        };
        trace!("adding clock_cycles:{} + {}", self.registers.clock_cycles(), clock_cycles);
        self.registers.add_clock_cycles(clock_cycles);
    }    
}

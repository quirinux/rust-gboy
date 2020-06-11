use super::*;
use super::super::*;
use super::optcode::OptCode;

impl Cpu {
    pub(crate) fn execute(&mut self, optcode: &OptCode) {

        match *optcode {
            OptCode::NOP => self.nop(),
            OptCode::LDnn(rt, a, b) => self.ld_nn(rt, a, b),
            OptCode::XORn(rt) => self.xor_n(rt),
            OptCode::LDDHLA => self.ldd_hl_a(),
            OptCode::LDNNn(rt, v) => self.ld_nn_n(rt, v),
            OptCode::LDNn(f, t) => self.ld_n_n(f, t),
            OptCode::LDNnAddress(f, t) => self.ld_n_address(f, t),
            OptCode::LDCA => self.ld_c_a(),
            OptCode::INCn(rt) => self.inc_n(rt) ,
            OptCode::INCnn(rt) => self.inc_nn(rt) ,
            OptCode::LDnA(rt) => self.ld_n_a(rt),
            OptCode::LDnnA(a, b) => self.ld_nn_a(a, b),
            OptCode::LDHnA(a) => self.ldh_n_a(a),
            OptCode::LDHAn(a) => self.ldh_a_n(a),
            OptCode::Call(a, b) => self.call(a, b),
            OptCode::PushNN(rt) => self.push_nn(rt),
            OptCode::PopNN(rt) => self.pop_nn(rt),
            OptCode::JRCCn(rt, f, a) => self.jr_cc_n(rt, f, a),
            OptCode::RLn(rt) => self.rl_n(rt),
            OptCode::DecN(rt) => self.dec_n(rt),
            OptCode::LDIHLA => self.ldi_nn_address_n(RegisterType::A, RegisterType::HL),
            OptCode::RET => self.ret(),
            OptCode::SUBn(rt) => self.sub_n(rt),
            OptCode::CPn(rt) => self.cp_n(rt),
            OptCode::CPnAddress(rt) => self.cp_n_address(rt),
            OptCode::CPnValue(v) => self.cp_n_value(v),
            OptCode::JRn(a) => self.jr_n(a),
            OptCode::JPnn(a, b) => self.jp_nn(a, b),
            OptCode::ADDnn(d, o) => self.add_nn(d, o),
            OptCode::DI => self.di(),
            OptCode::EI => self.ei(),
            
            OptCode::None(opt) => {                
                warn!("instruction not found: {:#x} at pc:{:#x}", opt, self.registers.pc.value());
                self.controls.quit = true;
            },


            // CBs
            OptCode::CBBit7H => self.cb_bit_7_h(),
            OptCode::CBRLn(rt) => self.cb_rl_n(rt),
            
            OptCode::CBNone(opt) => {                
                warn!("CB instruction not found: {:#x} at pc:{:#x}", opt, self.registers.pc.value());
                self.controls.quit = true;
            },            
        }
    }

    fn nop(&mut self) {
        debug!("nop");
    }
    
    fn ld_nn(&mut self, rt: RegisterType, a: u8, b: u8) {
        let address = util::join_bytes(a, b);
        debug!("ld_nn => rt:{:?} a:{:#x}, b:{:#x} address:{:#x}", rt, a, b, address);
        match rt {
            RegisterType::SP => self.registers.sp_goto(address as usize),
            RegisterType::HL => self.registers.set_hl(address),
            RegisterType::DE => self.registers.set_de(address),
            RegisterType::BC => self.registers.set_bc(address),
            _ => panic!("instruction not found: {:?}", rt),
        }
    }

    fn xor_n(&mut self, rt: RegisterType) {
        debug!("xor_n => rt:{:?} a:{:#x}", rt, self.registers.a);
        self.registers.a ^= match rt {
            RegisterType::A => self.registers.a,
            RegisterType::B => self.registers.b,
            RegisterType::C => self.registers.c,
            RegisterType::D => self.registers.d,
            RegisterType::E => self.registers.e,
            RegisterType::F => self.registers.f.value(),
            RegisterType::G => self.registers.g,
            RegisterType::H => self.registers.h,
            RegisterType::L => self.registers.l,
            _ => panic!("instruction not found: {:?}", rt),
        };        
        debug!("a:{:#x}", self.registers.a);
        self.registers.f.reset();
        if self.registers.a == 0x0 {
            self.registers.f.set_zero();
        }
    }

    fn ldd_hl_a(&mut self) {
        let hl = self.registers.hl();
        debug!("ldd_hl_a => hl:{:#x} a:{:#x}", hl, self.registers.a);
        // self.memory.write(hl as usize, self.registers.a);
        self.memory[hl] = self.registers.a;
        self.registers.dec_hl();
    }

    fn jr_cc_n(&mut self, flag_register: FlagRegisterType, flag_state: u8, steps: i8) {
        debug!("jr_cc_n => register:{:?} flag_state:{} steps:{}", flag_register, flag_state, steps);
        debug!("F => Z:{:#x} S:{:#x} H:{:#x} C:{:#x} ", self.registers.f.zero_flag(), self.registers.f.subtract_flag(), self.registers.f.half_carry_flag(), self.registers.f.carry_flag(), );
        if self.registers.f.get(flag_register) == flag_state {
            // let new_pc = (self.registers.pc.next() as isize + steps as isize) as usize;
            // self.jump(new_pc);
            self.jr_n(steps);
            //self.controls.quit = false;
        }
    }    

    fn ld_nn_n(&mut self, rt: RegisterType, value: u8) {
        debug!("ld_nn_n rt:{:?} value:{:#x}", rt, value);
        self.registers.set(&rt, value)
    }

    fn ld_n_n(&mut self, from: RegisterType, to: RegisterType) {
        debug!("ld_n_n from:{:?} to:{:?}", from, to);
        let value = self.registers.get(&from);
        debug!("value:{:#x}", value);
        self.ld_nn_n(to, value);
    }

    fn ld_nn_a(&mut self, a: u8, b: u8) {
        let address = util::join_bytes(a, b);
        debug!("ld_nn_a a:{:#x} b:{:#x}, address:{:#x}", a, b, address);
        //self.memory.write(address as usize, self.registers.a);
        self.memory[address] = self.registers.a;
        //self.controls.quit = true;
    }

    fn ld_n_address(&mut self, from: RegisterType, to:RegisterType) {
        debug!("ld_n_address from:{:?} to:{:?}", from, to);
        let address = self.registers.get2(&from);
        // let value = self.memory.read(address as usize);
        let value = self.memory[address];
        debug!("address{:#x} value:{:#x}", address, value);
        self.ld_nn_n(to, value);
    }

    fn ldi_nn_address_n(&mut self, from: RegisterType, to:RegisterType) {
        debug!("ld_nn_address_n from:{:?} to:{:?}", from, to);
        let value = self.registers.get(&from);
        let address = self.registers.get2(&to);
        trace!("{:?}:{:#x} => ({:?}):{:#x}", from, value, to, address);
        // self.memory.write(address as usize, value);
        self.memory[address] = value;
        self.registers.dec2(&to);

        //self.controls.quit = true;
    }
    
    fn ld_c_a(&mut self) {
        // let address = self.memory.io_initial_address() + self.registers.c as usize;
        let address = FF00 + self.registers.c as usize;
        error!("ld_c_a c:{:#x} a:{:#x} address:{:#x}", self.registers.c, self.registers.a, address);
        // self.memory.write_ff00(self.registers.c as usize, self.registers.a)
        self.memory[address] = self.registers.a;
        //self.controls.quit = true;
    }

    fn inc_n(&mut self, rt: RegisterType) {
        let value = self.registers.get(&rt);
        let new_value = value.wrapping_add(1);
        debug!("inc_n rt:{:?} value:{:#x} new_value:{:#x} ", rt, value, new_value);
        self.registers.set(&rt, new_value);
        self.registers.f.set(FlagRegisterType::Zero, new_value == 0);
        self.registers.f.unset_sub();
        self.registers.f.set(FlagRegisterType::Half, util::half_carry_occured(new_value)); // carry bit 3 for BCD
        
        //self.controls.quit = true;
    }    

    fn dec_n(&mut self, rt: RegisterType) {
        debug!("dec_n rt:{:?}", rt);
        let value = self.registers.get(&rt);
        let new_value = value.wrapping_sub(1);
        trace!("value:{:#x} new_value:{:#x}", value, new_value);
        self.registers.set(&rt, new_value);
        self.registers.f.set(FlagRegisterType::Zero, new_value == 0x0);
        self.registers.f.set_sub();
        self.registers.f.set(FlagRegisterType::Half, !util::half_carry_occured(value)); // carry bit 3 for BCD        

        //self.controls.quit = true;
    }

    fn inc_nn(&mut self, rt: RegisterType) {
        debug!("inc_nn rt:{:?}", rt);
        // let mut value = self.registers.get2(&rt);
        // value = value.wrapping_add(1); 
        // self.registers.set2(&rt, value);
        self.registers.inc2(&rt);
        //self.controls.quit = true;
    }    

    fn dec_nn(&mut self, rt: RegisterType) {
        debug!("dec_nn rt:{:?}", rt);
        // let mut value = self.registers.get2(&rt);
        // value = value.wrapping_sub(1); 
        // self.registers.set2(&rt, value);
        self.registers.dec2(&rt);
        //self.controls.quit = true;
    }    

    fn ld_n_a(&mut self, rt: RegisterType) {
        debug!("ld_n_a rt:{:?}", rt);
        let address = self.registers.get2(&rt);
        debug!("address:{:#x}", address);
        // self.memory.write(address as usize, self.registers.a);
        self.memory[address] = self.registers.a;

        //self.controls.quit = true;
    }

    fn ldh_n_a(&mut self, value: u8) {
        error!("ldh_n_a value:0xFF00+{:#x} = a:{:#x}", value, self.registers.a);
        // self.memory.write_ff00(value as usize, self.registers.a);
        self.memory[FF00 + value as usize] = self.registers.a;

        //self.controls.quit = true;
    }

    fn ldh_a_n(&mut self, value: u8) {
        error!("ldh_a_n value:0xFF00+{:#x}", value);
        // self.registers.a = self.memory.read_ff00(value as usize);
        self.registers.a = self.memory[FF00 + value as usize];
        error!("ff00 read a:{:#x}", self.registers.a);
        //self.controls.quit = true;
    }
    
    fn call(&mut self, a: u8, b: u8) {
        let address = util::join_bytes(a, b);
        debug!("call a:{:#x} b:{:#x } address:{:#x} pc:{:#x}", a, b, address, self.registers.pc.value());
        let (a, b) = util::split_bytes(self.registers.pc.value() as u16);
        self.stack_push(a as u8);
        self.stack_push(b as u8);

        self.jump(address as usize);

        //self.controls.quit = true;
    }

    fn push_nn(&mut self, rt: RegisterType) {
        debug!("push nn rt:{:?}", rt);
        let values = self.registers.get2(&rt);
        let (b, a) = util::split_bytes(values);
        debug!("push nn a:{:#x} b:{:#x }", a, b);
        self.stack_push(a);
        self.stack_push(b);

        //self.controls.quit = true;
    }

    fn pop_nn(&mut self, rt: RegisterType) {
        let a = self.stack_pop();
        let b = self.stack_pop();
        let value = util::join_bytes(a, b);
        debug!("pop r:{:?} nn a:{:#x} b:{:#x} value:{:#x}", rt, a, b, value);
        self.registers.set2(&rt, value);

        //self.controls.quit = true;
    }

    fn ret(&mut self) {
        let a = self.stack_pop();
        let b = self.stack_pop();
        let address = util::join_bytes(b, a);
        debug!("ret a:{:#x} b:{:#x} address:0x{:04x}", a, b, address);
        self.jump(address as usize);
        //self.controls.quit = true;
    }
    
    fn rl_n(&mut self, rt: RegisterType) {
        debug!("rl_n rt:{:?}", rt);
        let value = self.registers.get(&rt);
        let bit7state = self.registers.bitstate(&rt, 7);
        let new_value = value.rotate_left(1);
        debug!("value:{:#x}(0b{:08b}) bit7state:{:#b} new_value:{:#x}(0b{:08b})", value, value, bit7state, new_value, new_value);
        self.registers.set(&rt, new_value);

        self.registers.f.set(FlagRegisterType::Zero, new_value == 0x0);
        self.registers.f.unset_sub();
        self.registers.f.unset_half_carry();
        self.registers.f.set(FlagRegisterType::Carry, bit7state == 0x1);

        debug!("F:0b{:08b}", self.registers.f.value());
        //self.controls.quit = true;
    }

    fn cp_n(&mut self, rt: RegisterType) {
        let value = self.registers.get(&rt);
        debug!("cp_n rt:{:?} value::{:#x}", rt, value);
        self.cp_n_value(value);
    }

    fn cp_n_address(&mut self, rt: RegisterType) {
        let address = self.registers.get2(&rt);
        // let value = self.memory.read(address as usize);
        let value = self.memory[address];
        debug!("cp_n_address rt:{:?} address:{:#x} value:{:#x}", rt, address, value);
        self.cp_n_value(value);
    }

    fn cp_n_value(&mut self, value: u8) {
        let a = self.registers.a;
        debug!("cp_n_value a:{:#x} value:{:#x}", a, value);
        self.registers.f.set(FlagRegisterType::Zero, a == value);
        self.registers.f.set_sub();
        self.registers.f.set(FlagRegisterType::Half, util::half_carry_occured(a));
        self.registers.f.set(FlagRegisterType::Carry, a < value);
        debug!("F:0b{:08b}", self.registers.f.value());

        //self.controls.quit = true;
    }
    
    fn sub_n(&mut self, rt: RegisterType) {
        debug!("sub_n rt:{:?}", rt);
        //self.dec_n(rt);
        let value = self.registers.get(&rt);
        let new_a_value = self.registers.a.wrapping_sub(value);
        trace!("a:{:#x} - value:{:#x} = new_a_value:{:#x}", self.registers.a, value, new_a_value);
        self.registers.f.set(FlagRegisterType::Zero, new_a_value == 0x0);
        self.registers.f.set_sub();
        self.registers.f.set(FlagRegisterType::Half, !util::half_carry_occured(new_a_value)); // carry bit 3 for BCD
        self.registers.f.set(FlagRegisterType::Carry, self.registers.a < new_a_value); 
        self.registers.a = new_a_value;

        //self.controls.quit = true;
    }

    fn jr_n(&mut self, steps: i8) {
        // let new_pc = (self.registers.pc.next() as isize + steps as isize) as usize;
        let new_pc = (self.registers.pc.value() as isize + steps as isize) as usize;
        debug!("jr_n a:{:#x} = {} pc:{:#x} new_pc:{:#x}", steps, steps, self.registers.pc.value(), new_pc);
        
        self.jump(new_pc);
        
        //self.controls.quit = true;
    }

    fn jp_nn(&mut self, a: u8, b: u8) {
        debug!("jp_nn a:{:#x} b:{:#x}", a, b);
        let address = util::join_bytes(a, b);
        self.jump(address as usize);
    }

    fn add_nn(&mut self, destination: RegisterType, origin: RegisterType) {
        debug!("add_nn d:{:?} o:{:?}", destination, origin);
        let dst_value = self.registers.get(&destination);
        let src_value = match origin {
            RegisterType::HL => {
                let address = self.registers.get2(&origin);
                // let value = self.memory.read(address as usize);
                let value = self.memory[address];
                trace!("({:?}):{:#x}={:#x}", origin, address, value);
                value
            },
            _ => self.registers.get(&destination),
        };
        
        let (new_value, has_overflowed) = dst_value.overflowing_add(src_value);
        debug!("dst_value:{:#x} src_value:{:#x}, new_value:{:#x} overflowed:{}", dst_value, src_value, new_value, has_overflowed);
        self.registers.set(&destination, new_value);

        self.registers.f.set(FlagRegisterType::Zero, new_value == 0x0);
        self.registers.f.set_sub();
        self.registers.f.set(FlagRegisterType::Half, util::half_carry_occured(new_value));
        self.registers.f.set(FlagRegisterType::Carry, has_overflowed);
        

        //self.controls.quit = true;
    }

    fn di(&mut self) {
        debug!("DI");
        self.memory[0xFFFF as usize] = 0x0;
    }

    fn ei(&mut self) {
        debug!("EI");
        self.memory[0xFFFF as usize] = 0b0001_1111;
    }
    
    /// Internals
    fn stack_push(&mut self, value: u8) {
        trace!("stack_push sp:{:#} value:{:#x}", self.registers.sp, value);
        // self.memory.write(self.registers.sp, value);
        self.memory[self.registers.sp] = value;
        self.registers.sp -= 1;
    }
    
    fn stack_pop(&mut self) -> u8 {
        self.registers.sp += 1;
        // let value = self.memory.read(self.registers.sp);
        let value = self.memory[self.registers.sp];
        trace!("stack_pop sp:{:#x} value:{:#x}", self.registers.sp, value);
        value
    }

    fn jump(&mut self, address: usize) {
        debug!("jump address:{:#x}", address);
        self.registers.pc.jump(address);
        //self.controls.has_jumped = true;
    }
    

    /// CB instructions
    fn cb_bit_7_h(&mut self) {
        let bitstate = self.registers.bitstate(&RegisterType::H, 7);
        debug!("bit_7_h H:{:#b} = {:#x} = bitstate:{}", self.registers.h, self.registers.h, bitstate);
        self.registers.f.set(FlagRegisterType::Zero, bitstate == 0);
        debug!("F:{:#b}", self.registers.f.value());
        //panic!("to soon");
    }

    fn cb_rl_n(&mut self, rt: RegisterType) {
        debug!("cb_rl_n rt:{:?}", rt);
        self.rl_n(rt);
        //self.controls.quit = true;
    }
}


use super::super::util;

const ZERO_FLAG_POSITION: u8 = 7;
const SUBT_FLAG_POSITION: u8 = 6;
const HALF_FLAG_POSITION: u8 = 5;
const CARRY_FLAG_POSITION: u8 = 4;

#[derive(Debug, Copy, Clone)]
pub enum FlagRegisterType {
    Zero,
    Subt,
    Half,
    Carry,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct FlagRegister {
    value: u8,
}

impl FlagRegister {

    fn mask(&mut self, position: u8) -> u8 {
        (0b0000_0001 << position)
    }

    fn set_mask(&mut self, position: u8) {
        trace!("set_mask position:#{}", position);
        let pos = self.mask(position);
        trace!("old value:0b{:08b} | 0b{:08b} = 0b{:08b}", self.value, pos, self.value | pos);
        self.value |= self.mask(position)
    }

    fn unset_mask(&mut self, position: u8) {
        trace!("unset_mask position:#{}", position);
        let pos = self.mask(position);
        trace!("old value:0b{:08b} & 0b{:08b} = 0b{:08b}", self.value, !pos, self.value & !pos);
        self.value &= !self.mask(position)
    }    

    fn flag_state(&mut self, position: u8) -> u8 {
        (self.value >> position) & 0b1
    }
    
    pub fn set_zero(&mut self) {
        trace!("set_zero");
        self.set_mask(ZERO_FLAG_POSITION)
    }
    pub fn unset_zero(&mut self) {
        trace!("reset_zero");
        self.unset_mask(ZERO_FLAG_POSITION)
    }
    pub fn set_sub(&mut self) {
        trace!("set_sub");
        self.set_mask(SUBT_FLAG_POSITION)
    }
    pub fn unset_sub(&mut self) {
        trace!("reset_sub");
        self.unset_mask(SUBT_FLAG_POSITION)   
    }
    pub fn set_half_carry(&mut self) {
        trace!("set_half_carry");
        self.set_mask(HALF_FLAG_POSITION)
    }
    pub fn unset_half_carry(&mut self) {
        trace!("reset_half_carry");
        self.unset_mask(HALF_FLAG_POSITION)  
    }
    pub fn set_carry(&mut self) {
        trace!("set_carry");
        self.set_mask(CARRY_FLAG_POSITION)
    }
    pub fn unset_carry(&mut self) {
        trace!("reset_carry");
        self.unset_mask(CARRY_FLAG_POSITION)
    }

    pub fn zero_flag(&mut self) -> u8 { self.flag_state(ZERO_FLAG_POSITION) }
    pub fn subtract_flag(&mut self) -> u8 { self.flag_state(SUBT_FLAG_POSITION) }
    pub fn half_carry_flag(&mut self) -> u8 { self.flag_state(HALF_FLAG_POSITION) }
    pub fn carry_flag(&mut self) -> u8 { self.flag_state(CARRY_FLAG_POSITION) }

    pub fn reset(&mut self) {
        self.unset_zero();
        self.unset_sub();
        self.unset_half_carry();
        self.unset_carry();
    }

    pub fn value(&mut self) -> u8 {
        self.value
    }

    pub fn set(&mut self, frt: FlagRegisterType, state: bool) {
        let flag_position = match frt {
            FlagRegisterType::Zero => ZERO_FLAG_POSITION,
            FlagRegisterType::Subt => SUBT_FLAG_POSITION,
            FlagRegisterType::Half => HALF_FLAG_POSITION,
            FlagRegisterType::Carry => CARRY_FLAG_POSITION,            
        };
        trace!("set flag:{:?} state:{}", frt, state);
        if state {
            trace!("setting");
            self.set_mask(flag_position);
        } else {
            trace!("resetting");
            self.unset_mask(flag_position);
        }
    }
    
    pub fn get(&mut self, frt: FlagRegisterType) -> u8 {
        match frt {
            FlagRegisterType::Zero => self.zero_flag(),
            FlagRegisterType::Subt => self.subtract_flag(),
            FlagRegisterType::Half => self.half_carry_flag(),
            FlagRegisterType::Carry => self.carry_flag(),
        }
    }
    
}
#[derive(Debug, Copy, Clone)]
pub enum RegisterType {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    L,

    AF,
    BC,
    DE,
    HL,
    
    PC,
    SP,
    CC,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct ProgramCounter {
    value: usize,
}

impl ProgramCounter {

    pub fn value(&mut self) -> usize {
        self.value
    }
    
    /// walks PC to next positios and returns its new positon
    pub fn walk(&mut self) -> usize {
        self.value = self.next();
        self.value
    }

    /// picks up next PC positoin
    pub fn next(&mut self) -> usize {
        self.value + 1
    }

    pub fn jump(&mut self, address: usize) {
        self.value = address;
    }

}

#[derive(Default, Debug, Copy, Clone)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagRegister,
    pub g: u8,
    pub h: u8,
    pub l: u8,

    pub pc: ProgramCounter,
    pub sp: usize,
    clock_cycles: usize,
}

impl Registers {
    
    pub fn set_hl(&mut self, value: u16) {
        self.set2(&RegisterType::HL, value)
    }

    pub fn set_de(&mut self, value: u16) {
        self.set2(&RegisterType::DE, value)
    }

    pub fn set_bc(&mut self, value: u16) {
        self.set2(&RegisterType::BC, value)
    }

    pub fn af(&mut self) -> u16 {
        util::join_bytes(self.a, self.f.value())
    }
    pub fn bc(&mut self) -> u16 {
        util::join_bytes(self.b, self.c)
    }
    pub fn de(&mut self) -> u16 {
        util::join_bytes(self.d, self.e)
    }
    
    pub fn hl(&mut self) -> u16 {
        util::join_bytes(self.h, self.l)
    }

    pub fn dec_hl(&mut self) {
        self.dec2(&RegisterType::HL);
    }
    
    pub fn sp_goto(&mut self, address: usize) {
        self.sp = address;
    }

    pub fn add_clock_cycles(&mut self, cost: usize) {
        self.clock_cycles += cost;
    }

    pub fn clock_cycles(&mut self) -> usize {
        self.clock_cycles
    }

    pub fn bitstate(&mut self, rt: &RegisterType, position: u8) -> u8 {
        let value = self.get(rt);
        (value >> position) & 0b1
    }

    pub fn set_bitstate(&mut self, rt: &RegisterType, position: u8) {
        let value = self.get(rt);
        self.set(rt, (value >> position) | 0b1);
    }

    
    pub fn set(&mut self, rt: &RegisterType, value: u8) {
        trace!("setting RT::{:?} => {:#x}", rt, value);
        match rt {
            RegisterType::A => self.a = value,
            RegisterType::B => self.b = value,
            RegisterType::C => self.c = value,
            RegisterType::D => self.d = value,
            RegisterType::E => self.e = value,
            RegisterType::H => self.h = value,
            RegisterType::L => self.l = value,
            _ => panic!("register set not found => {:?}", rt),
        }
    }

    pub fn set2(&mut self, rt: &RegisterType, value: u16) {
        let (a, b) = util::split_bytes(value);
        match rt {
            RegisterType::DE =>{
                self.d = a;
                self.e = b;
            },
            RegisterType::HL =>{
                self.h = a;
                self.l = b;
            },
            RegisterType::BC =>{
                self.b = a;
                self.c = b;
            },
            _ => panic!("set2 => invalid register rt:{:?}", rt),            
        }
    }
    
    pub fn get(&mut self, rt: &RegisterType) -> u8 {
        match rt {
            RegisterType::A => self.a,
            RegisterType::B => self.b,
            RegisterType::C => self.c,
            RegisterType::D => self.d,
            RegisterType::E => self.e,
            RegisterType::G => self.g,
            RegisterType::H => self.h,
            RegisterType::L => self.l,
            _ => panic!("flag_state => invalid register rt:{:?}", rt),
        }
    }

    pub fn get2(&mut self, rt: &RegisterType) -> u16 {
            match rt {
                RegisterType::AF => self.af(),
                RegisterType::BC => self.bc(),
                RegisterType::DE => self.de(),
                RegisterType::HL => self.hl(),
                _ => panic!("flag_state => invalid register rt:{:?}", rt),
            }
    }

    pub fn inc2(&mut self, rt: &RegisterType) {
        let mut value = self.get2(rt);
        value += 1;
        self.set2(rt, value);
    }
    pub fn dec2(&mut self, rt: &RegisterType) {
        let mut value = self.get2(rt);
        value -= 1;
        self.set2(rt, value);
    }

}

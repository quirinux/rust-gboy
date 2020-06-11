

use super::*;
use super::super::*;
use crate::gboy::cpu::optcode::OptCode;

#[derive(Debug)]
pub enum CpuDebuggerMessage {
    CatridgeTitle(String),
    OptCode(OptCode),
    Registers {
        a: u8,
        b: u8,
        c: u8,
        d: u8,
        e: u8,
        g: u8,
        h: u8,
        l: u8,
        pc: usize,
        sp: usize,
        //ime: bool,
    },
    Display {
        stat: u8,
        scy: u8,
        scx: u8,
        wy: u8,
        wx: u8,
        ly: u8,
        lyc: u8,
    },
}

pub trait CpuDebugger {
    // method called once at startup
    fn initialize(&mut self);
    
    // method to be called on every CPU tick
    fn tick(&mut self);

    // method called right before quitting
    fn quit(&mut self);

    // method to be called whenever a debug message is sent
    fn message(&mut self, msg: CpuDebuggerMessage);
}

impl Cpu {
    pub(crate) fn initialize_debugger(&mut self) {
        if let Some(d) = &mut self.debugger {
            d.message(CpuDebuggerMessage::CatridgeTitle(self.memory.cartridge_title()));
            d.initialize();
        }
    }

    pub(crate) fn tick_debugger(&mut self) {
        if let Some(d) = &mut self.debugger {
            d.message(CpuDebuggerMessage::Registers{
                a: self.registers.a,
                b: self.registers.b,
                c: self.registers.c,
                d: self.registers.d,
                e: self.registers.e,
                g: self.registers.g,
                h: self.registers.h,
                l: self.registers.l,
                pc: self.registers.pc.value(),
                sp: self.registers.sp,
                //ime: self.registers.ime,
            });
            d.message(CpuDebuggerMessage::Display{
                stat: self.memory[0xff00 + 0x41 as usize],
                scy: self.memory[0xff00 + 0x42 as usize],
                scx: self.memory[0xff00 + 0x43 as usize],
                ly: self.memory[0xff00 + 0x44 as usize],
                lyc: self.memory[0xff00 + 0x45 as usize],
                wy: self.memory[0xff00 + 0x4A as usize],
                wx: self.memory[0xff00 + 0x4B as usize],
            });
            
            d.tick();
        }
    }
    
    pub(crate) fn quit_debugger(&mut self) {
        if let Some(d) = &mut self.debugger {
            d.quit();
        }
    }
    
    pub(crate) fn message_debugger(&mut self, msg: CpuDebuggerMessage) {
        if let Some(d) = &mut self.debugger {
            d.message(msg);
        }
    }    
}


pub mod optcode;
pub mod debugger;
mod registers;
mod instructions;
mod timer;
mod interrupt;

pub use debugger::*;
use super::cartridge::*;
use super::memorybus::*;
use registers::*;
use timer::*;

use log::Level;

const ROM_INITIAL_ADDRESS: usize = 0x100;
const SP_INITIAL_ADDRESS: usize = 0xFFFE;
const FF00: usize = 0xFF00;

#[derive(Default)]
struct Control {
    old_pc: usize,
    game_booted: bool,
    quit: bool,
    div_control: usize,
}

pub struct Cpu {
    memory: Memory,
    registers: Registers,
    controls: Control,
    debugger: Option<Box<dyn CpuDebugger>>,
}

pub fn initialize(gamerom: Vec<u8>, mut debugger: Option<Box<dyn CpuDebugger>>) -> Cpu {

    let mut cartridge = super::cartridge::new(gamerom);
    let mut memory = super::memorybus::new(cartridge);
    
    Cpu {
        memory: memory,
        registers: Registers::default(),
        controls: Control::default(),
        debugger: debugger,
    }
}


impl Cpu {


    fn load_bootrom(&mut self, bootrom: Option<Vec<u8>>) {
        match bootrom {
            Some(b) => {
                for i in 0..b.len() {
                    // self.memory.write(0x0 + i, b[i]);
                    //info!("{:#x} => {:#x}", i, b[i]);
                    self.memory[0x0 + i] = b[i];
                }
            },

            None => {
                // disables bootrom
                self.registers.a = 0x01;
                // unmaps bootrom
                // self.memory.write_ff00(0x50, self.registers.a);
                self.memory[FF00 + 0x50] = self.registers.a;
                // setting PC to initial ROM address, usyally let on 0x100 by bootrom
                self.registers.pc.jump(ROM_INITIAL_ADDRESS);
                // clearing up stack pointer
                self.registers.sp = SP_INITIAL_ADDRESS;
                // setting boot flag on
                self.controls.game_booted = true;
            }            
        }
    }
    
    fn load_gamerom(&mut self) {
        trace!("loading gamerom");
        self.memory.load_cartridge_rom();
        trace!("gamerom fully loaded to ram");
        //panic!("");
    }
    
    pub fn bootup(&mut self, bootrom: Option<Vec<u8>>) {
        self.memory.load_cartridge_header();
        self.load_bootrom(bootrom);
        
        // self.memory.write_ff00(0x44, 0x90); //enabling VBlank
        self.memory[FF00 + 0x44] = 0x90; //enabling VBlank
        //self.registers.ime = true; // Enabling Interrupt Master Enable Flag

        self.initialize_debugger();
    }

    fn run_bootrom(&mut self) {
        loop {
            trace!("looping bootrom");
            self.tick_debugger();
            
            // checking if bootrom is released
            // if self.registers.pc.value() == ROM_INITIAL_ADDRESS && self.memory[FF00 + 0x50] == 0x1 {
            if self.memory[FF00 + 0x50] == 0x1 { // game header validated
                self.controls.game_booted = true;
            }            
            
            // checking if coming from a jump or a walk
            if self.controls.old_pc == self.registers.pc.value() {
                trace!("PC walked to => {:#X}", self.registers.pc.value());
            } else {
                trace!("PC jumped to => {:#X}", self.registers.pc.value());
                match self.registers.pc.value() {
                    0xE9 => {
                        error!("Probably lock down due logo mismatch, quitting");
                        self.controls.quit = true;
                    },
                    0xFA => {
                        error!("Probably lock down due checksum mismatch, quitting");
                        self.controls.quit = true;
                    },
                    _ => {},
                }
            }
            
            let inst = self.read_instruction();
            trace!("instruction loaded => {:#x}", inst);
            let optcode = self.decode(inst);
            // updating old pc control before calling instruction
            self.controls.old_pc = self.registers.pc.value();
            self.message_debugger(CpuDebuggerMessage::OptCode(optcode.clone()));
            self.execute(&optcode);
            self.timer_tick(&optcode);
                
            if self.controls.quit || self.controls.game_booted {
                info!("quitting bootrom loop");
                break;
            }
        }
    }
    
    fn run_game(&mut self) {
        loop {
            trace!("looping gamerom");
            self.tick_debugger();
            
            // checking if coming from a jump or a walk
            if self.controls.old_pc == self.registers.pc.value() {
                trace!("PC walked to => {:#X}", self.registers.pc.value());
            } else {
                trace!("PC jumped to => {:#X}", self.registers.pc.value());
            }
            
            let inst = self.read_instruction();
            trace!("instruction loaded => {:#x}", inst);
            let optcode = self.decode(inst);
            // updating old pc control before calling instruction
            self.controls.old_pc = self.registers.pc.value();
            self.message_debugger(CpuDebuggerMessage::OptCode(optcode.clone()));
            self.execute(&optcode);
            self.timer_tick(&optcode);

            if self.registers.pc.value() == 0x237 {
                info!("quitting due PC value");
                self.controls.quit = true;
            }
             
            if self.controls.quit {
                info!("game looping quitting");
                self.quit_debugger();
                break;
            }

        }
    }

    pub fn run(&mut self) {
        if !self.controls.game_booted {
            self.run_bootrom();
        } else {
            info!("skipping bootrom");
        }

        if self.controls.game_booted && !self.controls.quit {
            self.load_gamerom();
            self.run_game();
        } else {
            error!("skipping gamerom, probably something went wrong on bootrom process");
        }
        
        if log_enabled!(log::Level::Debug) {
            self.dump();
        }        
    }
    
    fn read_instruction(&mut self) -> u8 {
        let m = self.memory[self.registers.pc.value()];
        self.registers.pc.walk();
        m
    }


    fn dump(&mut self) {
        println!("======     CARTRIDGE     =======");
        println!("Cartridge Title: {}", self.memory.cartridge_title());
        println!("Destination Code: {}", self.memory.destination_code());
        println!("Licensee Code: {}", self.memory.licensee_code());
        println!("Type: {}", self.memory.cartridge_type());
        let (c, s) = self.memory.cartridge_header_checksum();
        println!("Header Checksum: {:#x} calculated: {:#x}", c, s);

        println!("======     REGISTERS     =======");
        println!("A == {:#x} ", self.registers.a);
        println!("B == {:#x} ", self.registers.b);
        println!("C == {:#x} ", self.registers.c);
        println!("D == {:#x} ", self.registers.d);
        println!("E == {:#x} ", self.registers.e);
        println!("F => Z:{:#x} S:{:#x} H:{:#x} C:{:#x} ", self.registers.f.zero_flag(), self.registers.f.subtract_flag(), self.registers.f.half_carry_flag(), self.registers.f.carry_flag(), );
        println!("G == {:#x} ", self.registers.g);
        println!("H == {:#x} ", self.registers.h);
        println!("L == {:#x} ", self.registers.l);
        println!("PC == {:#x} ", self.registers.pc.value());
        println!("SP == {:#x} ", self.registers.sp);
        //println!("IME == {} ", self.registers.ime);

        println!("======     STACK     =======");
        for i in self.registers.sp..self.memory.memory_size() {
            println!("{:#x} == {:#x}", i, self.memory[i]);
        }

        println!("======     TIMERS     =======");
        println!("DIV == {:#x}", self.memory[FF00 + 0x04]);
        println!("TIMA == {:#x}", self.memory[FF00 + 0x05]);
        println!("TMA == {:#x}", self.memory[FF00 + 0x06]);
        println!("TAC == {:#x}", self.memory[FF00 + 0x07]);
        println!("CC == {}", self.registers.clock_cycles());

        println!("======     INTERRUPTS     =======");
        println!("IF FF0F == 0b{:08b}", self.memory[FF00 + 0x0F]);
        println!("IE FFFF == 0b{:08b}", self.memory[FF00 + 0xFF]);
        println!("LCDC FF40 == 0b{:08b}", self.memory[FF00 + 0x40]);
        println!("STAT FF41 == 0b{:08b}", self.memory[FF00 + 0x41]);
        
    }
}

use std::path::PathBuf;
use structopt::StructOpt;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::{thread};

#[macro_use]
extern crate log;
extern crate env_logger;
use env_logger::Env;
use log::{Record, LevelFilter};
use env_logger::Builder;

mod gboy;

const NONE_LOG_LEVEL: usize = 0;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Log level, will increase log level if passed multiple times: error, warn, info, debug, trace
    #[structopt(long, short, parse(from_occurrences))]
    debug: usize,

    /// boot room, it's skipped if none is passed in
    #[structopt(long, short, parse(from_os_str))]
    bootrom: Option<PathBuf>,

    /// enable tui debugger
    #[structopt(long)]
    debugger: bool,
    
    /// rom to emulate 
    #[structopt(parse(from_os_str))]
    gamerom: PathBuf,

}

fn log_level(lvl: usize) -> String {
    let levels = ["none", "error", "warn", "info", "debug", "trace"];
    if lvl >= levels.len(){
        levels[levels.len() - 1].to_string()
    } else {
        levels[lvl].to_string()
    }
}

fn load_file(path: PathBuf) -> Result<Vec<u8>, io::Error> {
   let mut file = File::open(path)?;
   let mut program_buffer = Vec::<u8>::new();
   file.read_to_end(&mut program_buffer)?;
   Ok(program_buffer)
}

fn main() {
    let opt = Opt::from_args();
    let mut debugger : Option<Box<dyn gboy::cpu::CpuDebugger>> = if opt.debugger {
        match gboy::debugger::initialize() {
            Ok(d) => Some(Box::new(d)),
            Err(e) => {
                println!("Could not open debugger => {}", e);
                None
            }
        }
    } else {
        None
    };
    
    if debugger.is_some() {
        //env_logger::from_env(Env::default().default_filter_or(log_level(NONE_LOG_LEVEL))).init();
    } else {
        env_logger::from_env(Env::default().default_filter_or(log_level(opt.debug))).init();
    }
    
    debug!("{:?}", opt); 
    let boot_rom = if opt.bootrom.is_some() {
        match load_file(opt.bootrom.unwrap()) {
            Ok(b) => Some(b),
            Err(e) => panic!("{}", e),
        }
    } else { 
        None 
    };

    let game_rom = match load_file(opt.gamerom){
        Ok(g) => g,
        Err(e) => panic!("{}", e),
    };

    trace!("boot_rom => {:?}", boot_rom);

    let mut console = gboy::cpu::initialize(game_rom, debugger);
    console.bootup(boot_rom);
    console.run();

}

use std::io;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Terminal;


use super::cpu::*;
use crate::gboy::cpu::debugger::*;
use crate::gboy::cpu::optcode::OptCode;


#[derive(Default, Clone, Copy)]
struct Cpu {
    pc: usize,
    sp: usize,
    opcode: OptCode,
    reg_a: u8,
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_e: u8,
    reg_h: u8,
    reg_g: u8,
    reg_l: u8,
    //reg_ime: bool,

    dis_stat: u8,
    dis_scy: u8,
    dis_scx: u8,
    dis_ly: u8,
    dis_lyc: u8,
    dis_wy: u8,
    dis_wx: u8,
}

pub struct Debugger {
    terminal: Terminal<TermionBackend<AlternateScreen<termion::raw::RawTerminal<io::Stdout>>>>,
    cpu: Cpu,
}

pub fn initialize() -> Result<impl CpuDebugger, io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    //let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    
    Ok(
        Debugger {
            terminal: terminal,
            cpu: Cpu::default(),
    })
}

impl Debugger {
    fn render(&mut self) -> Result<(), io::Error> {
        let cpu = self.cpu.clone();
        
        self.terminal.draw(|mut f| {
            let size = f.size();
            Block::default()
                //.style(Style::default().bg(Color::White))
                .render(&mut f, size);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                    ]
                        .as_ref(),
                )
                .split(size);

            let registers = [
                Text::raw(format!("A => {:#x}\n", cpu.reg_a)),
                Text::raw(format!("B => {:#x}\n", cpu.reg_b)),
                Text::raw(format!("C => {:#x}\n", cpu.reg_c)),
                Text::raw(format!("D => {:#x}\n", cpu.reg_d)),
                Text::raw(format!("E => {:#x}\n", cpu.reg_e)),
                Text::raw(format!("G => {:#x}\n", cpu.reg_g)),
                Text::raw(format!("H => {:#x}\n", cpu.reg_h)),
                Text::raw(format!("L => {:#x}\n", cpu.reg_l)),
                //Text::raw(format!("IME => {}\n", cpu.reg_ime)),
                Text::raw(format!("PC => {:#x}\n", cpu.pc)),
                Text::raw(format!("SP => {:#x}\n", cpu.sp)),
                Text::raw(format!("OpCode => {:?}\n", cpu.opcode)),                
            ];

            let display = [
                Text::raw(format!("STAT => {:#x}\n", cpu.dis_stat)),
                Text::raw(format!("WY => {:#x}\n", cpu.dis_wy)),
                Text::raw(format!("WX => {:#x}\n", cpu.dis_wx)),
                Text::raw(format!("SCY => {:#x}\n", cpu.dis_scy)),
                Text::raw(format!("SCX => {:#x}\n", cpu.dis_scx)),
                Text::raw(format!("LY => {:#x}\n", cpu.dis_ly)),
                Text::raw(format!("LYC => {:#x}\n", cpu.dis_lyc)),
            ];
            
            let block = Block::default()
                .borders(Borders::ALL)
                .title_style(Style::default().modifier(Modifier::BOLD));
            Paragraph::new(registers.iter())
                .block(block.clone().title("REGISTERS"))
                .alignment(Alignment::Left)
                .render(&mut f, chunks[0]);
            Paragraph::new(display.iter())
                .block(block.clone().title("DISPLAY"))
                .alignment(Alignment::Left)
                .wrap(true)
                .render(&mut f, chunks[1]);
            // Paragraph::new(text.iter())
            //     .block(block.clone().title("Center, wrap"))
            //     .alignment(Alignment::Center)
            //     .wrap(true)
            //     //.scroll(scroll)
            //     .render(&mut f, chunks[2]);
 
            
        })
    }
    
    
}

impl CpuDebugger for Debugger {
    
    // method called once at startup
    fn initialize(&mut self) {
        //println!("initialize");
        self.render();
    }
    
    // method to be called on every CPU tick
    fn tick(&mut self) {
        //println!("tick");
        self.render();

    }

    // method called right before quitting
    fn quit(&mut self){
        //println!("quit");
        self.render();

    }

    // method to be called whenever a debug message is sent
    fn message(&mut self, msg: CpuDebuggerMessage) {
        match msg {
            // CpuDebuggerMessage::Registers{sp, pc, a, b, c, d, e, h, g, l, ime } =>{
            CpuDebuggerMessage::Registers{sp, pc, a, b, c, d, e, h, g, l } =>{
                self.cpu.pc = pc;
                self.cpu.sp = sp;
                self.cpu.reg_a = a;
                self.cpu.reg_b = b;
                self.cpu.reg_c = c;
                self.cpu.reg_d = d;
                self.cpu.reg_e = e;
                self.cpu.reg_h = h;
                self.cpu.reg_g = g;
                self.cpu.reg_l = l;
                //self.cpu.reg_ime = ime;
                
            },
            CpuDebuggerMessage::OptCode(v) => self.cpu.opcode = v,
            CpuDebuggerMessage::Display{stat, wy, wx, ly, lyc, scy, scx } =>{
                self.cpu.dis_stat = stat;
                self.cpu.dis_wy = wy;
                self.cpu.dis_wx = wx;
                self.cpu.dis_ly = ly;
                self.cpu.dis_lyc = lyc;
                self.cpu.dis_scy = scy;
                self.cpu.dis_scx = scx;
            },
            _ => {},
        }
    }

}

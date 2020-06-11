use std::ops::{Index, IndexMut};

use super::super::cartridge::*;
const MEMORY_SIZE: usize = 0xFFFF;

pub struct Memory {
    ram: [u8; MEMORY_SIZE],
    cartridge: Cartridge,

    // Internals
    ime: u8,
}

pub fn new(cartridge: Cartridge) -> Memory {
    Memory {
        ram: [0; MEMORY_SIZE],
        cartridge: cartridge,

        // internals
        ime: 0x0,
    }
}

impl<T> Index<T> for Memory where T: Into<usize>{
    type Output = u8;
    fn index(&self, i: T) -> &Self::Output {
        //&self.ram[i.into()]
        let idx = &i.into();
        match *idx {
            0xFFFF => &self.ime,
            _ => &self.ram[*idx as usize],
        }
    }
}

impl<T> IndexMut<T> for Memory where T: Into<usize>{
    fn index_mut(&mut self, i: T) -> &mut Self::Output {
        let idx = &i.into();
        match *idx {
            0xFFFF => &mut self.ime,
            //0xFF0F => panic!(""),
            _ => &mut self.ram[*idx as usize],
        }        
    }
}

impl Memory {
    pub fn memory_size(&mut self) -> usize {
        MEMORY_SIZE
    }

    pub fn load_cartridge_header(&mut self) {
        let s = self.cartridge.header_start();
        let e = self.cartridge.header_end();
        for i in s..e {
            self.ram[i] = self.cartridge.read(i);
        }
    }

    pub fn load_cartridge_rom(&mut self) {
        let s = self.cartridge.header_start();
        let e = self.cartridge.rom_len();
        for i in s..e {
            self.ram[i] = self.cartridge.read(i);
        }
    }

    pub fn cartridge_title(&mut self) -> String {
        self.cartridge.title()
    }
    
    pub fn destination_code(&mut self) -> String {
        self.cartridge.destination_code()
    }

    pub fn licensee_code(&mut self) -> String {
        self.cartridge.licensee_code()
    }

    pub fn cartridge_type(&mut self) -> String {
        self.cartridge.cart_type()
    }

    pub fn cartridge_header_checksum(&mut self) -> (u8, u8) {
        (self.cartridge.header_checksum(), self.cartridge.calculate_header_checksum())
    }
}

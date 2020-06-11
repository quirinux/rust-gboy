


pub struct Cartridge {
    game_rom: Vec<u8>,

}

pub fn new(gamerom: Vec<u8>) -> Cartridge {
    
    Cartridge {
        game_rom: gamerom,
    }
}


impl Cartridge {

    pub fn rom_len(&mut self) -> usize {
        self.game_rom.len()
    }

    pub fn header_start(&mut self) -> usize {
        0x0
    }

    pub fn header_end(&mut self) -> usize {
        0x14F
    }
    
    pub fn title(&mut self) -> String {
        String::from_utf8(self.game_rom[0x134..0x13E].to_vec()).unwrap()        
    }

    pub fn destination_code(&mut self) -> String {
        match self.read(0x14A) {
            0x0 => "Japanese".to_string(),
            0x1 => "Non-Japanese".to_string(),
            _ => format!("Not defined"),
        }        
    }

    pub fn licensee_code(&mut self) -> String {
        match self.read(0x14B) {
            0x0 => "None",
            0x1 => "Nintendo R&D1",
            _ => {
                debug!("Not defined = > {:#x}", self.read(0x14B));
                "Not defined"
            },
        }.to_string()
    }

    pub fn cart_type(&mut self) -> String {
        match self.read(0x147) {
            0x0 => "Rom Only",
            _ => {
                debug!("Not defined = > {:#x}", self.read(0x147));
                "Not defined"
            },
        }.to_string()
    }

    pub fn read(&mut self, address: usize) -> u8 {
        self.game_rom[address]
    }

    pub fn header_checksum(&mut self) -> u8 {
        self.read(0x14D)
    }

    pub fn calculate_header_checksum(&mut self) -> u8 {
        let mut x: u8 = 0;
        for i in 0x134..=0x14C {
            x = x.wrapping_sub(self.read(i)).wrapping_sub(1);
        }
        x
    }
}

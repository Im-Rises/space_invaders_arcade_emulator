use std::fs::File;
use std::io::{Error, Read};

const MEMORY_SIZE: usize = 0x4000;

pub struct Mmu {
    memory: [u8; MEMORY_SIZE],
}

impl Mmu {
    pub fn new() -> Self {
        let mut mmu = Mmu {
            memory: [0; MEMORY_SIZE],
        };

        let array_h: [u8; 0x800] = read_rom("./game_roms/invaders.h").unwrap();
        let array_g: [u8; 0x800] = read_rom("./game_roms/invaders.g").unwrap();
        let array_f: [u8; 0x800] = read_rom("./game_roms/invaders.f").unwrap();
        let array_e: [u8; 0x800] = read_rom("./game_roms/invaders.e").unwrap();
        mmu.memory[0..0x800].clone_from_slice(&array_h);
        mmu.memory[0x800..0x1000].clone_from_slice(&array_g);
        mmu.memory[0x1000..0x1800].clone_from_slice(&array_f);
        mmu.memory[0x1800..0x2000].clone_from_slice(&array_e);

        mmu
    }

    pub fn new_debug() -> Self {
        let mut mmu = Mmu {
            memory: [0; MEMORY_SIZE],
        };

        let debug_rom: [u8; 0x800] = read_rom("test_roms/TST8080.COM").unwrap();
        mmu.memory[0x100..(0x100 + debug_rom.len())].copy_from_slice(&debug_rom);
        mmu.memory[0x7] = 0xc9;
        mmu.memory[0x5] = 0xd3;
        mmu.memory[0x6] = 0x01;
        mmu.memory[0x0] = 0xd3;

        mmu
    }

    pub fn read(&self, address: u16) -> u8 {
        if address < 0x4000 {
            self.memory[address as usize]
        } else if address < 0x8000 {
            self.memory[(address - 0x4000) as usize]
        } else if address < 0xC000 {
            self.memory[(address - 0x8000) as usize]
        } else {
            self.memory[(address - 0xC000) as usize]
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        if address < 0x4000 {
            self.memory[address as usize] = data;
        } else if address < 0x8000 {
            self.memory[(address - 0x4000) as usize] = data;
        } else if address < 0xC000 {
            self.memory[(address - 0x8000) as usize] = data;
        } else {
            self.memory[(address - 0xC000) as usize] = data;
        }
    }

    pub fn get_vram(&self) -> &[u8] {
        &self.memory[0x2400..0x4000]
    }
}

fn read_rom(rom_path: &str) -> Result<[u8; 0x800], Error> {
    let mut f = File::open(rom_path)?;
    let mut buffer: [u8; 0x800] = [0; 0x800];
    let size = f.read(&mut buffer)?;
    if size > 0x800 {
        panic!("Error: File is incorrectly sized {}", 0x800);
    }
    Ok(buffer)
}

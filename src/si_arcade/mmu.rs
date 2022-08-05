use std::fs::File;
use std::io;
use std::io::{Error, Read};

// const MEMORY_SIZE: usize = 0x4000;
const MEMORY_SIZE: usize = 0x10000;

#[allow(dead_code)]
const DEBUG_MEMORY_SIZE: usize = 0x10000;

pub struct Mmu {
    memory: Vec<u8>,
}

impl Mmu {
    pub fn new() -> Self {
        let mut mmu = Mmu {
            memory: vec![0; MEMORY_SIZE],
        };

        let array_h: [u8; 0x800] = space_invaders_rom("./game_roms/invaders.h").unwrap();
        let array_g: [u8; 0x800] = space_invaders_rom("./game_roms/invaders.g").unwrap();
        let array_f: [u8; 0x800] = space_invaders_rom("./game_roms/invaders.f").unwrap();
        let array_e: [u8; 0x800] = space_invaders_rom("./game_roms/invaders.e").unwrap();
        mmu.memory[0..0x800].clone_from_slice(&array_h);
        mmu.memory[0x800..0x1000].clone_from_slice(&array_g);
        mmu.memory[0x1000..0x1800].clone_from_slice(&array_f);
        mmu.memory[0x1800..0x2000].clone_from_slice(&array_e);

        mmu
    }

    #[allow(dead_code)]
    pub fn new_debug(rom_path: &str) -> Self {
        let mut mmu = Mmu {
            memory: vec![0; DEBUG_MEMORY_SIZE],
        };

        let debug_rom_and_size = read_complete_rom(rom_path).unwrap();
        mmu.memory[0x100..(0x100 + debug_rom_and_size.1)].copy_from_slice(&debug_rom_and_size.0);

        // inject "out 0,a" at 0x0000 (signal to stop the test)
        mmu.memory[0x0000] = 0xD3;
        mmu.memory[0x0001] = 0x00;
        // inject "out 1,a" at 0x0005 (signal to output some characters)
        mmu.memory[0x0005] = 0xD3;
        mmu.memory[0x0006] = 0x01;
        mmu.memory[0x0007] = 0xC9;

        mmu
    }
    //Wrong implementation of RAM banking, I implemented like Rom/Ram banking
    //It is only Ram Banking
    pub fn read(&self, address: u16) -> u8 {
        // if address < 0x4000 {
        self.memory[address as usize]
        // } else if address < 0x6000 {
        //     self.memory[(address - 0x2000) as usize]
        // } else if address < 0x8000 {
        //     self.memory[(address - 0x4000) as usize]
        // } else if address < 0xA000 {
        //     self.memory[(address - 0x6000) as usize]
        // } else if address < 0xC000 {
        //     self.memory[(address - 0x8000) as usize]
        // } else if address < 0xE000 {
        //     self.memory[(address - 0xA000) as usize]
        // } else {
        //     self.memory[(address - 0xE000) as usize]
        // }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        // if address < 0x4000 {
        self.memory[address as usize] = data;
        // } else if address < 0x6000 {
        //     self.memory[(address - 0x2000) as usize] = data;
        // } else if address < 0x8000 {
        //     self.memory[(address - 0x4000) as usize] = data;
        // } else if address < 0xA000 {
        //     self.memory[(address - 0x6000) as usize] = data;
        // } else if address < 0xC000 {
        //     self.memory[(address - 0x8000) as usize] = data;
        // } else if address < 0xE000 {
        //     self.memory[(address - 0xA000) as usize] = data;
        // } else {
        //     self.memory[(address - 0xE000) as usize] = data;
        // }
    }

    pub fn get_vram(&self) -> &[u8] {
        &self.memory[0x2400..0x4000]
    }
}

fn space_invaders_rom(rom_path: &str) -> Result<[u8; 0x800], Error> {
    let mut f = File::open(rom_path)?;
    let mut buffer: [u8; 0x800] = [0; 0x800];
    let size = f.read(&mut buffer)?;
    if size > 0x800 {
        panic!("Error: File is incorrectly sized {}", 0x800);
    }
    Ok(buffer)
}

fn read_complete_rom(rom_path: &str) -> io::Result<(Vec<u8>, usize)> {
    let mut f = File::open(rom_path)?;
    let mut buffer = Vec::new();
    let size = f.read_to_end(&mut buffer)?;
    Ok((buffer, size))
}

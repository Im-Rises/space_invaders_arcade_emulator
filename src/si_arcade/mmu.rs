use std::fs::File;
// use std::fs::File;
// use std::{fs, io};
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

const MEMORY_SIZE: usize = 0x4000;

pub struct Mmu {
    // memory: Vec<u8>,
    // romh: Vec<u8>,
    // romg: Vec<u8>,
    // romf: Vec<u8>,
    // rome: Vec<u8>,
    memory: [u8; MEMORY_SIZE],
}

impl Mmu {
    pub fn new(roms_path: &str) -> Mmu {
        let mut mmu = Mmu {
            memory: [0; MEMORY_SIZE],
            // memory: vec![0; MEMORY_SIZE],
        };

        let h = include_bytes!("invaders.h");
        let g = include_bytes!("invaders.g");
        let f = include_bytes!("invaders.f");
        let e = include_bytes!("invaders.e");

        mmu.memory[0..(h.len())].copy_from_slice(h);
        mmu.memory[0x800..(h.len() + 0x800)].copy_from_slice(g);
        mmu.memory[0x1000..(h.len() + 0x1000)].copy_from_slice(f);
        mmu.memory[0x1800..(h.len() + 0x1800)].copy_from_slice(e);

        mmu
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }
}

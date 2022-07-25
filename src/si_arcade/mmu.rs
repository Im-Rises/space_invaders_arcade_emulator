use std::fs::File;
use std::io::{BufReader, Read};
use std::{fs, io};

const MEMORY_SIZE: usize = 0x4000;

pub struct Mmu {
    memory: Vec<u8>,
    // romh: Vec<u8>,
    // romg: Vec<u8>,
    // romf: Vec<u8>,
    // rome: Vec<u8>,
    // memory: [u8; MEMORY_SIZE],
}

impl Mmu {
    pub fn new(roms_path: &str) -> Mmu {
        let mut mmu = Mmu {
            // memory: [0; MEMORY_SIZE],
            memory: vec![0; MEMORY_SIZE],
        };
        // let image = std::fs::read("TST8080.COM").ok().unwrap();
        // let mut i = 0;
        // for n in image {
        //     mmu.memory[0x100 + i] = n;
        //     i += 1;
        // }

        // let mut file = BufReader::new(File::open("TST8080.COM"));
        // file.read_exact(&mut mmu.memory[0x100..(0x100 + 0x7FF)])
        //     .expect("TODO: panic message");

        // let mut f = File::open("TST8080.COM")?;
        //
        // // read exactly 10 bytes
        // let size = f.read(&mut mmu.memory[0x100..]);
        // Ok(());

        // io::copy()

        // mmu.memory[0x100..] = *fs::read("TST8080.COM").expect("TODO: panic message");

        mmu
    }

    fn load_rom(&self, rompath: &str) {}

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }
}

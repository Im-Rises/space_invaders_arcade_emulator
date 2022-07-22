const MEMORY_SIZE: usize = 0x4000;

pub struct Mmu {
    // romh: Vec<u8>,
    // romg: Vec<u8>,
    // romf: Vec<u8>,
    // rome: Vec<u8>,
    memory: [u8; MEMORY_SIZE],
}

impl Mmu {
    pub fn new(roms_path: &str) -> Mmu {
        Mmu {
            // romh: vec![],
            // romg: vec![],
            // romf: vec![],
            // rome: vec![],
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn load_roms(&self) {}

    fn load_rom(&self, path: String) {}

    pub fn read(&self, address: u16) -> u8 {
        10
    }

    pub fn write(&self) {}
}

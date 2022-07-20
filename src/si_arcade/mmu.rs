pub struct Mmu {
    romh: Vec<u8>,
    romg: Vec<u8>,
    romf: Vec<u8>,
    rome: Vec<u8>,
}

impl Mmu {
    pub fn new(roms_path: &str) -> Mmu {
        Mmu{
            romh: vec![],
            romg: vec![],
            romf: vec![],
            rome: vec![]
        }
    }

    pub fn load_roms(&self) {}

    fn load_rom(&self, path: String) {}

    pub fn read(&self){

    }

    pub fn write(&self){

    }
}

pub struct Cpu {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    stat: u16,
    opcode: u8,
    cycles: u8,
}

impl Cpu {
    fn clock(&self)
    {
        if self.cycles == 0
        {
            let opcode = fetch();
            self.compute_opcode(opcode);
        }
        cycles -= 1;
    }

    fn compute_opcode(&self, opcode: u8)
    {
        //Implement a big switch case here.
        match opcode
        {
            _ =>()
        }
    }

    fn fetch(&self) -> u8
    {
        // 10
    }

    fn pair_registers(&self)
    {}

    fn unpair_registers(&self)
    {}
}

// enum Opocdes{
//     NOP(0x00),
//
// }

impl Cpu {
    // List all the opcodes function here

    fn mov_r1_r2(&self, mut r1: &u8, mut r2: &u8)
    {
        r1 = r2;
    }

    fn mov_m_r()
    {

    }

    fn nop()
    {
        // Do nothing
    }

    //
    // fn lxi_b_d16(&self)
    // {
    //     let mut fetched :u16 = self.fetch() as u16;
    //     fetched<<= 8;
    //     fetched |= fetched();
    // }
}
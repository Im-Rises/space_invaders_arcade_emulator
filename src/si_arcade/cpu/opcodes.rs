struct Opcode {}

impl Opcode {
    // List all the opcodes function here

    fn mov_r1_r2(&self, mut r1: u8, r2: u8)
    {
        r1 = r2;
    }

    fn mov_m_r(&self) {}

    fn nop(&self) {
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

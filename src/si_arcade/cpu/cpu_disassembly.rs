// Thanks to https://github.com/superzazu/8080/blob/master/i8080.c

#[allow(dead_code)]
pub const DISASSEMBLY_TABLE: [&str; 256] = [
    "nop", "lxi b,#", "stax b", "inx b", "inr b", "dcr b", "mvi b,#", "rlc", "ill", "dad b", "ldax b", "dcx b",
    "inr c", "dcr c", "mvi c,#", "rrc", "ill", "lxi d,#", "stax d", "inx d", "inr d", "dcr d", "mvi d,#", "ral", "ill",
    "dad d", "ldax d", "dcx d", "inr e", "dcr e", "mvi e,#", "rar", "ill", "lxi h,#", "shld", "inx h", "inr h",
    "dcr h", "mvi h,#", "daa", "ill", "dad h", "lhld", "dcx h", "inr l", "dcr l", "mvi l,#", "cma", "ill", "lxi sp,#",
    "sta $", "inx sp", "inr M", "dcr M", "mvi M,#", "stc", "ill", "dad sp", "lda $", "dcx sp", "inr a", "dcr a",
    "mvi a,#", "cmc", "mov b,b", "mov b,c", "mov b,d", "mov b,e", "mov b,h", "mov b,l", "mov b,M", "mov b,a",
    "mov c,b", "mov c,c", "mov c,d", "mov c,e", "mov c,h", "mov c,l", "mov c,M", "mov c,a", "mov d,b", "mov d,c",
    "mov d,d", "mov d,e", "mov d,h", "mov d,l", "mov d,M", "mov d,a", "mov e,b", "mov e,c", "mov e,d", "mov e,e",
    "mov e,h", "mov e,l", "mov e,M", "mov e,a", "mov h,b", "mov h,c", "mov h,d", "mov h,e", "mov h,h", "mov h,l",
    "mov h,M", "mov h,a", "mov l,b", "mov l,c", "mov l,d", "mov l,e", "mov l,h", "mov l,l", "mov l,M", "mov l,a",
    "mov M,b", "mov M,c", "mov M,d", "mov M,e", "mov M,h", "mov M,l", "hlt", "mov M,a", "mov a,b", "mov a,c",
    "mov a,d", "mov a,e", "mov a,h", "mov a,l", "mov a,M", "mov a,a", "add b", "add c", "add d", "add e", "add h",
    "add l", "add M", "add a", "adc b", "adc c", "adc d", "adc e", "adc h", "adc l", "adc M", "adc a", "sub b",
    "sub c", "sub d", "sub e", "sub h", "sub l", "sub M", "sub a", "sbb b", "sbb c", "sbb d", "sbb e", "sbb h",
    "sbb l", "sbb M", "sbb a", "ana b", "ana c", "ana d", "ana e", "ana h", "ana l", "ana M", "ana a", "xra b",
    "xra c", "xra d", "xra e", "xra h", "xra l", "xra M", "xra a", "ora b", "ora c", "ora d", "ora e", "ora h",
    "ora l", "ora M", "ora a", "cmp b", "cmp c", "cmp d", "cmp e", "cmp h", "cmp l", "cmp M", "cmp a", "rnz", "pop b",
    "jnz $", "jmp $", "cnz $", "push b", "adi #", "rst 0", "rz", "ret", "jz $", "ill", "cz $", "call $", "aci #",
    "rst 1", "rnc", "pop d", "jnc $", "out p", "cnc $", "push d", "sui #", "rst 2", "rc", "ill", "jc $", "in p",
    "cc $", "ill", "sbi #", "rst 3", "rpo", "pop h", "jpo $", "xthl", "cpo $", "push h", "ani #", "rst 4", "rpe",
    "pchl", "jpe $", "xchg", "cpe $", "ill", "xri #", "rst 5", "rp", "pop psw", "jp $", "di", "cp $", "push psw",
    "ori #", "rst 6", "rm", "sphl", "jm $", "ei", "cm $", "ill", "cpi #", "rst 7",
];

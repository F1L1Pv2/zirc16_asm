use phf::phf_map;

// TODO: add pseudo instructions

pub const REGISTERS: &[&'static str] = &["r0","r1","r2","r3","r4","r5","r6","r7","r8","r9","r10","r11","r12","r13","r14","r15"];

pub const REGISTERS_TO_VAL: phf::Map<&'static str, usize> = phf_map!{
    "r0" => 0,
    "r1" => 1,
    "r2" => 2,
    "r3" => 3,
    "r4" => 4,
    "r5" => 5,
    "r6" => 6,
    "r7" => 7,
    "r8" => 8,
    "r9" => 9,
    "r10" => 10,
    "r11" => 11,
    "r12" => 12,
    "r13" => 13,
    "r14" => 14,
    "r15" => 15,
};

//format {(type)(count in bits)}
// possible types:
// R - Register
// IMM - Immediate
// C - Cond
// E - Extra

pub const INSTRUCTIONS: phf::Map<&'static str, &'static str> = phf_map!{
    "hlt" => "00000 0000 0000 000",
    "add" => "00001 {R4} {R4} 00{E1}",
    "sub" => "00010 {R4} {R4} 00{E1}",
    "adi" => "00011 {R4} {IMM6} 0",
    "and" => "00100 {R4} {R4} 000",
    "nor" => "00101 {R4} {R4} 000",
    "xor" => "00110 {R4} {R4} 000",
    "rsh" => "00111 {R4} 0000 00{E1}",
    "cmp" => "01000 {R4} {R4} 0{E2}",
    "lim" => "01001 {R4} {IMM6} 0",
    "lui" => "01010 {IMM10} 0",
    "psh" => "01011 {R4} {R4} 000",
    "pop" => "01100 {R4} {R4} 000",
    "str" => "01101 {R4} {R4} 000",
    "lod" => "01110 {R4} {R4} 000",
    "brc" => "01111 {C4} {IMM6} {E1}",
    "bal" => "10000 {C4} {R4} 00{E1}",
    "ret" => "10001 0000 0000 000"
};


pub const PSEUDO_INSTRUCTIONS: phf::Map<&'static str, &'static str> = phf_map!{
    "mov a,b" => "
        xor a, a
        xor a, b
    ",
    "lsh rd" => "
        add rd, r0
    "
};
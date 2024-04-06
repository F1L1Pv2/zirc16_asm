use phf::phf_map;

//format {(type)(count in bits)}
// possible types:
// IMM - Immediate
// E - Extra
// + declared

pub const TYPES: phf::Map<&'static str, phf::Map<&'static str, usize>> = phf_map!{
    "R" => REGISTERS_TO_VAL,
    "C" => CONDITIONS_TO_VAL,
    "SR" => SPECIAL_REGISTERS_TO_VAL,
};

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

pub const SPECIAL_REGISTERS_TO_VAL: phf::Map<&'static str, usize> = phf_map!{
    "sptr" => 0,
};

pub const CONDITIONS_TO_VAL: phf::Map<&'static str, usize> = phf_map!{
    "z" =>  0b0000,
    "nz" => 0b0001,
    "c" =>  0b0010,
    "nc" => 0b0011,
    "p" =>  0b0100,
    "np" => 0b0101,
    "s" =>  0b0110,
    "ns" => 0b0111,
    "o" =>  0b1000,
    "no" => 0b1001,
    "e" =>  0b1010,
    "ne" => 0b1011,
    "ge" => 0b1100,
    "l" => 0b1101,
    "t" => 0b1110
};


pub const INSTRUCTIONS: phf::Map<&'static str, &'static str> = phf_map!{
    "hlt" => "00000 0000 0000 000",
    "add" => "00001 {R4} {R4} 00{E1}",
    "adi" => "00010 {R4} {IMM6} 0",
    "sub" => "00011 {R4} {R4} 00{E1}",
    "mul" => "00100 {R4} {R4} 00{E1}",
    "div" => "00101 {R4} {R4} 0{E2}",
    "fsg" => "00110 {R4} 0000 000",
    "and" => "00111 {R4} {R4} 000",
    "nor" => "01000 {R4} {R4} 000",
    "xor" => "01001 {R4} {R4} 000",
    "lsh" => "01010 {R4} {IMM4} 00{E1}",
    "rot" => "01011 {R4} {IMM4} 00{E1}",
    "mov" => "01100 {R4} {R4} 000",
    "cmp" => "01101 {R4} {R4} 0{E2}",
    "cmi" => "01110 {R4} {IMM6} {E1}",
    "lim" => "01111 {R4} {IMM6} 0",
    "lui" => "10000 {IMM10} 0",
    "ssr" => "10001 {SR4} {R4} 000",
    "lsr" => "10010 {R4} {SR4} 000",
    "psh" => "10011 {R4} 0000 000",
    "pop" => "10100 {R4} 0000 000",
    "str" => "10101 {R4} {R4} 000",
    "lod" => "10110 {R4} {R4} 000",
    "brc" => "10111 {C4} {IMM6} {E1}",
    "bri" => "11000 {C4} {R4} 00{E1}",
    "cal" => "11001 {C4} {R4} 00{E1}",
    "ret" => "11010 0000 0000 000"
};

// Closure ops
// +  add
// -  subtract
// *  multuply
// /  divide
// &  bitwise and
// |  bitwise or
// ^  bitwise xor
// << bitshift left
// >> bitshift right

pub const PSEUDO_INSTRUCTIONS: phf::Map<&'static str, &'static str> = phf_map!{
    "limb rd, a" => "
        lui (a >> 6)
        lim rd, (a & 0x3F)
    "
};

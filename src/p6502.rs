use crate::bus::BUS;

pub enum FLAGS6502{
    C = (1 << 0), //C Carry flag 0 false 1 true
    Z = (1 << 1), //Z Zero flag 0 result not zero 1 result zero
    I = (1 << 2), //I IRQ disable flag 0 enable 1 disable
    D = (1 << 3), //D Decimal Mode Flag 0 false 1 true
    B = (1 << 4), //B Break Command Flag 0 No break 1 Break
    U = (1 << 5), //U Unused
    V = (1 << 6), //V Overflow flag 0 false 1 true
    N = (1 << 7), //N Negative flag 0 positive 1 negative
}

struct INSTRUCTION{
    name:String,
    operate:fn(&mut P6502)->u8,
    addrmode:fn(&mut P6502)->u8,
    cycles:u8,
}

pub struct P6502{
   
    pub pc: u16, //accumulator
    pub stkp: u8, //stack pointer

    //8bit registers
    pub a: u8,
    pub x: u8,
    pub y: u8,

    pub status: u8, //NV_BDIZC

    pub fetched: u8,

    pub addr_abs: u16,
    pub addr_rel: u16,
    pub opcode: u8,
    pub cycles: u8,

    lookup:Vec<INSTRUCTION>,

    pub bus:BUS,
}
#[allow(non_snake_case)]
impl P6502{
    pub fn new ()->P6502{
        P6502{
            pc :0x0000,
            stkp : 0x00,
            a: 0,
            x: 0,
            y: 0,
            status: 0b0000_0000,

            fetched: 0x00,
            
            addr_abs: 0x0000,
            addr_rel: 0x00,
            opcode: 0x00,
            cycles: 0, 


            bus:BUS::new(),

            //lookup : vec![INSTRUCTION{name:String::from("BRK"), operate: P6502::BRK, addrmode:P6502::IMM, cycles:7}]

            lookup: vec![
                INSTRUCTION{name:String::from("BRK"), operate: P6502::BRK, addrmode: P6502::IMM, cycles:7 },INSTRUCTION{name:String::from("ORA"), operate: P6502::ORA, addrmode: P6502::IZX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:8 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:3 },INSTRUCTION{name:String::from("ORA"), operate: P6502::ORA, addrmode: P6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("ASL"), operate: P6502::ASL, addrmode: P6502::ZP0, cycles:5 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:5 },INSTRUCTION{name:String::from("PHP"), operate: P6502::PHP, addrmode: P6502::IMP, cycles:3 },INSTRUCTION{name:String::from("ORA"), operate: P6502::ORA, addrmode: P6502::IMM, cycles:2 },INSTRUCTION{name:String::from("ASL"), operate: P6502::ASL, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:4 },INSTRUCTION{name:String::from("ORA"), operate: P6502::ORA, addrmode: P6502::ABS, cycles:4 },INSTRUCTION{name:String::from("ASL"), operate: P6502::ASL, addrmode: P6502::ABS, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:6 },
                INSTRUCTION{name:String::from("BPL"), operate: P6502::BPL, addrmode: P6502::REL, cycles:2 },INSTRUCTION{name:String::from("ORA"), operate: P6502::ORA, addrmode: P6502::IZY, cycles:5 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:8 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:4 },INSTRUCTION{name:String::from("ORA"), operate: P6502::ORA, addrmode: P6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("ASL"), operate: P6502::ASL, addrmode: P6502::ZPX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:6 },INSTRUCTION{name:String::from("CLC"), operate: P6502::CLC, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("ORA"), operate: P6502::ORA, addrmode: P6502::ABY, cycles:4 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:7 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:4 },INSTRUCTION{name:String::from("ORA"), operate: P6502::ORA, addrmode: P6502::ABX, cycles:4 },INSTRUCTION{name:String::from("ASL"), operate: P6502::ASL, addrmode: P6502::ABX, cycles:7 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:7 },
                INSTRUCTION{name:String::from("JSR"), operate: P6502::JSR, addrmode: P6502::ABS, cycles:6 },INSTRUCTION{name:String::from("AND"), operate: P6502::AND, addrmode: P6502::IZX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:8 },INSTRUCTION{name:String::from("BIT"), operate: P6502::BIT, addrmode: P6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("AND"), operate: P6502::AND, addrmode: P6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("ROL"), operate: P6502::ROL, addrmode: P6502::ZP0, cycles:5 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:5 },INSTRUCTION{name:String::from("PLP"), operate: P6502::PLP, addrmode: P6502::IMP, cycles:4 },INSTRUCTION{name:String::from("AND"), operate: P6502::AND, addrmode: P6502::IMM, cycles:2 },INSTRUCTION{name:String::from("ROL"), operate: P6502::ROL, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("BIT"), operate: P6502::BIT, addrmode: P6502::ABS, cycles:4 },INSTRUCTION{name:String::from("AND"), operate: P6502::AND, addrmode: P6502::ABS, cycles:4 },INSTRUCTION{name:String::from("ROL"), operate: P6502::ROL, addrmode: P6502::ABS, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:6 },
                INSTRUCTION{name:String::from("BMI"), operate: P6502::BMI, addrmode: P6502::REL, cycles:2 },INSTRUCTION{name:String::from("AND"), operate: P6502::AND, addrmode: P6502::IZY, cycles:5 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:8 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:4 },INSTRUCTION{name:String::from("AND"), operate: P6502::AND, addrmode: P6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("ROL"), operate: P6502::ROL, addrmode: P6502::ZPX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:6 },INSTRUCTION{name:String::from("SEC"), operate: P6502::SEC, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("AND"), operate: P6502::AND, addrmode: P6502::ABY, cycles:4 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:7 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:4 },INSTRUCTION{name:String::from("AND"), operate: P6502::AND, addrmode: P6502::ABX, cycles:4 },INSTRUCTION{name:String::from("ROL"), operate: P6502::ROL, addrmode: P6502::ABX, cycles:7 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:7 },
                INSTRUCTION{name:String::from("RTI"), operate: P6502::RTI, addrmode: P6502::IMP, cycles:6 },INSTRUCTION{name:String::from("EOR"), operate: P6502::EOR, addrmode: P6502::IZX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:8 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:3 },INSTRUCTION{name:String::from("EOR"), operate: P6502::EOR, addrmode: P6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("LSR"), operate: P6502::LSR, addrmode: P6502::ZP0, cycles:5 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:5 },INSTRUCTION{name:String::from("PHA"), operate: P6502::PHA, addrmode: P6502::IMP, cycles:3 },INSTRUCTION{name:String::from("EOR"), operate: P6502::EOR, addrmode: P6502::IMM, cycles:2 },INSTRUCTION{name:String::from("LSR"), operate: P6502::LSR, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("JMP"), operate: P6502::JMP, addrmode: P6502::ABS, cycles:3 },INSTRUCTION{name:String::from("EOR"), operate: P6502::EOR, addrmode: P6502::ABS, cycles:4 },INSTRUCTION{name:String::from("LSR"), operate: P6502::LSR, addrmode: P6502::ABS, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:6 },
                INSTRUCTION{name:String::from("BVC"), operate: P6502::BVC, addrmode: P6502::REL, cycles:2 },INSTRUCTION{name:String::from("EOR"), operate: P6502::EOR, addrmode: P6502::IZY, cycles:5 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:8 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:4 },INSTRUCTION{name:String::from("EOR"), operate: P6502::EOR, addrmode: P6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("LSR"), operate: P6502::LSR, addrmode: P6502::ZPX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:6 },INSTRUCTION{name:String::from("CLI"), operate: P6502::CLI, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("EOR"), operate: P6502::EOR, addrmode: P6502::ABY, cycles:4 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:7 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:4 },INSTRUCTION{name:String::from("EOR"), operate: P6502::EOR, addrmode: P6502::ABX, cycles:4 },INSTRUCTION{name:String::from("LSR"), operate: P6502::LSR, addrmode: P6502::ABX, cycles:7 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:7 },
                INSTRUCTION{name:String::from("RTS"), operate: P6502::RTS, addrmode: P6502::IMP, cycles:6 },INSTRUCTION{name:String::from("ADC"), operate: P6502::ADC, addrmode: P6502::IZX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:8 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:3 },INSTRUCTION{name:String::from("ADC"), operate: P6502::ADC, addrmode: P6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("ROR"), operate: P6502::ROR, addrmode: P6502::ZP0, cycles:5 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:5 },INSTRUCTION{name:String::from("PLA"), operate: P6502::PLA, addrmode: P6502::IMP, cycles:4 },INSTRUCTION{name:String::from("ADC"), operate: P6502::ADC, addrmode: P6502::IMM, cycles:2 },INSTRUCTION{name:String::from("ROR"), operate: P6502::ROR, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("JMP"), operate: P6502::JMP, addrmode: P6502::IND, cycles:5 },INSTRUCTION{name:String::from("ADC"), operate: P6502::ADC, addrmode: P6502::ABS, cycles:4 },INSTRUCTION{name:String::from("ROR"), operate: P6502::ROR, addrmode: P6502::ABS, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:6 },
                INSTRUCTION{name:String::from("BVS"), operate: P6502::BVS, addrmode: P6502::REL, cycles:2 },INSTRUCTION{name:String::from("ADC"), operate: P6502::ADC, addrmode: P6502::IZY, cycles:5 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:8 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:4 },INSTRUCTION{name:String::from("ADC"), operate: P6502::ADC, addrmode: P6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("ROR"), operate: P6502::ROR, addrmode: P6502::ZPX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:6 },INSTRUCTION{name:String::from("SEI"), operate: P6502::SEI, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("ADC"), operate: P6502::ADC, addrmode: P6502::ABY, cycles:4 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:7 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:4 },INSTRUCTION{name:String::from("ADC"), operate: P6502::ADC, addrmode: P6502::ABX, cycles:4 },INSTRUCTION{name:String::from("ROR"), operate: P6502::ROR, addrmode: P6502::ABX, cycles:7 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:7 },
                INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("STA"), operate: P6502::STA, addrmode: P6502::IZX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:6 },INSTRUCTION{name:String::from("STY"), operate: P6502::STY, addrmode: P6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("STA"), operate: P6502::STA, addrmode: P6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("STX"), operate: P6502::STX, addrmode: P6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:3 },INSTRUCTION{name:String::from("DEY"), operate: P6502::DEY, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("TXA"), operate: P6502::TXA, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("STY"), operate: P6502::STY, addrmode: P6502::ABS, cycles:4 },INSTRUCTION{name:String::from("STA"), operate: P6502::STA, addrmode: P6502::ABS, cycles:4 },INSTRUCTION{name:String::from("STX"), operate: P6502::STX, addrmode: P6502::ABS, cycles:4 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:4 },
                INSTRUCTION{name:String::from("BCC"), operate: P6502::BCC, addrmode: P6502::REL, cycles:2 },INSTRUCTION{name:String::from("STA"), operate: P6502::STA, addrmode: P6502::IZY, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:6 },INSTRUCTION{name:String::from("STY"), operate: P6502::STY, addrmode: P6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("STA"), operate: P6502::STA, addrmode: P6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("STX"), operate: P6502::STX, addrmode: P6502::ZPY, cycles:4 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:4 },INSTRUCTION{name:String::from("TYA"), operate: P6502::TYA, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("STA"), operate: P6502::STA, addrmode: P6502::ABY, cycles:5 },INSTRUCTION{name:String::from("TXS"), operate: P6502::TXS, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:5 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:5 },INSTRUCTION{name:String::from("STA"), operate: P6502::STA, addrmode: P6502::ABX, cycles:5 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:5 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:5 },
                INSTRUCTION{name:String::from("LDY"), operate: P6502::LDY, addrmode: P6502::IMM, cycles:2 },INSTRUCTION{name:String::from("LDA"), operate: P6502::LDA, addrmode: P6502::IZX, cycles:6 },INSTRUCTION{name:String::from("LDX"), operate: P6502::LDX, addrmode: P6502::IMM, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:6 },INSTRUCTION{name:String::from("LDY"), operate: P6502::LDY, addrmode: P6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("LDA"), operate: P6502::LDA, addrmode: P6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("LDX"), operate: P6502::LDX, addrmode: P6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:3 },INSTRUCTION{name:String::from("TAY"), operate: P6502::TAY, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("LDA"), operate: P6502::LDA, addrmode: P6502::IMM, cycles:2 },INSTRUCTION{name:String::from("TAX"), operate: P6502::TAX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("LDY"), operate: P6502::LDY, addrmode: P6502::ABS, cycles:4 },INSTRUCTION{name:String::from("LDA"), operate: P6502::LDA, addrmode: P6502::ABS, cycles:4 },INSTRUCTION{name:String::from("LDX"), operate: P6502::LDX, addrmode: P6502::ABS, cycles:4 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:4 },
                INSTRUCTION{name:String::from("BCS"), operate: P6502::BCS, addrmode: P6502::REL, cycles:2 },INSTRUCTION{name:String::from("LDA"), operate: P6502::LDA, addrmode: P6502::IZY, cycles:5 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:5 },INSTRUCTION{name:String::from("LDY"), operate: P6502::LDY, addrmode: P6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("LDA"), operate: P6502::LDA, addrmode: P6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("LDX"), operate: P6502::LDX, addrmode: P6502::ZPY, cycles:4 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:4 },INSTRUCTION{name:String::from("CLV"), operate: P6502::CLV, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("LDA"), operate: P6502::LDA, addrmode: P6502::ABY, cycles:4 },INSTRUCTION{name:String::from("TSX"), operate: P6502::TSX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:4 },INSTRUCTION{name:String::from("LDY"), operate: P6502::LDY, addrmode: P6502::ABX, cycles:4 },INSTRUCTION{name:String::from("LDA"), operate: P6502::LDA, addrmode: P6502::ABX, cycles:4 },INSTRUCTION{name:String::from("LDX"), operate: P6502::LDX, addrmode: P6502::ABY, cycles:4 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:4 },
                INSTRUCTION{name:String::from("CPY"), operate: P6502::CPY, addrmode: P6502::IMM, cycles:2 },INSTRUCTION{name:String::from("CMP"), operate: P6502::CMP, addrmode: P6502::IZX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:8 },INSTRUCTION{name:String::from("CPY"), operate: P6502::CPY, addrmode: P6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("CMP"), operate: P6502::CMP, addrmode: P6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("DEC"), operate: P6502::DEC, addrmode: P6502::ZP0, cycles:5 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:5 },INSTRUCTION{name:String::from("INY"), operate: P6502::INY, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("CMP"), operate: P6502::CMP, addrmode: P6502::IMM, cycles:2 },INSTRUCTION{name:String::from("DEX"), operate: P6502::DEX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("CPY"), operate: P6502::CPY, addrmode: P6502::ABS, cycles:4 },INSTRUCTION{name:String::from("CMP"), operate: P6502::CMP, addrmode: P6502::ABS, cycles:4 },INSTRUCTION{name:String::from("DEC"), operate: P6502::DEC, addrmode: P6502::ABS, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:6 },
                INSTRUCTION{name:String::from("BNE"), operate: P6502::BNE, addrmode: P6502::REL, cycles:2 },INSTRUCTION{name:String::from("CMP"), operate: P6502::CMP, addrmode: P6502::IZY, cycles:5 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:8 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:4 },INSTRUCTION{name:String::from("CMP"), operate: P6502::CMP, addrmode: P6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("DEC"), operate: P6502::DEC, addrmode: P6502::ZPX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:6 },INSTRUCTION{name:String::from("CLD"), operate: P6502::CLD, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("CMP"), operate: P6502::CMP, addrmode: P6502::ABY, cycles:4 },INSTRUCTION{name:String::from("NOP"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:7 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:4 },INSTRUCTION{name:String::from("CMP"), operate: P6502::CMP, addrmode: P6502::ABX, cycles:4 },INSTRUCTION{name:String::from("DEC"), operate: P6502::DEC, addrmode: P6502::ABX, cycles:7 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:7 },
                INSTRUCTION{name:String::from("CPX"), operate: P6502::CPX, addrmode: P6502::IMM, cycles:2 },INSTRUCTION{name:String::from("SBC"), operate: P6502::SBC, addrmode: P6502::IZX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:8 },INSTRUCTION{name:String::from("CPX"), operate: P6502::CPX, addrmode: P6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("SBC"), operate: P6502::SBC, addrmode: P6502::ZP0, cycles:3 },INSTRUCTION{name:String::from("INC"), operate: P6502::INC, addrmode: P6502::ZP0, cycles:5 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:5 },INSTRUCTION{name:String::from("INX"), operate: P6502::INX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("SBC"), operate: P6502::SBC, addrmode: P6502::IMM, cycles:2 },INSTRUCTION{name:String::from("NOP"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::SBC, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("CPX"), operate: P6502::CPX, addrmode: P6502::ABS, cycles:4 },INSTRUCTION{name:String::from("SBC"), operate: P6502::SBC, addrmode: P6502::ABS, cycles:4 },INSTRUCTION{name:String::from("INC"), operate: P6502::INC, addrmode: P6502::ABS, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:6 },
                INSTRUCTION{name:String::from("BEQ"), operate: P6502::BEQ, addrmode: P6502::REL, cycles:2 },INSTRUCTION{name:String::from("SBC"), operate: P6502::SBC, addrmode: P6502::IZY, cycles:5 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:8 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:4 },INSTRUCTION{name:String::from("SBC"), operate: P6502::SBC, addrmode: P6502::ZPX, cycles:4 },INSTRUCTION{name:String::from("INC"), operate: P6502::INC, addrmode: P6502::ZPX, cycles:6 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:6 },INSTRUCTION{name:String::from("SED"), operate: P6502::SED, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("SBC"), operate: P6502::SBC, addrmode: P6502::ABY, cycles:4 },INSTRUCTION{name:String::from("NOP"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:2 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:7 },INSTRUCTION{name:String::from("???"), operate: P6502::NOP, addrmode: P6502::IMP, cycles:4 },INSTRUCTION{name:String::from("SBC"), operate: P6502::SBC, addrmode: P6502::ABX, cycles:4 },INSTRUCTION{name:String::from("INC"), operate: P6502::INC, addrmode: P6502::ABX, cycles:7 },INSTRUCTION{name:String::from("???"), operate: P6502::XXX, addrmode: P6502::IMP, cycles:7 },
            ]
            
        }
    }

//addressing modes

    fn IMP(&mut self)->u8{
        self.fetched = self.a;
        0
    }
    fn IMM(&mut self)->u8{
        self.addr_abs = self.pc;
        self.pc += 1;
        0
    }
    fn ZP0(&mut self)->u8{
        self.addr_abs = self.read(self.pc) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00ff;
        0
    }
    fn ZPX(&mut self)->u8{
        self.addr_abs = (self.read(self.pc) + self.x) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00ff;
        0
    }
    fn ZPY(&mut self)->u8{
        self.addr_abs = (self.read(self.pc) + self.y) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00ff;
        0
    }
    fn REL(&mut self)->u8{
        self.addr_rel = self.read(self.pc) as u16;
        self.pc += 1;
        if (self.addr_rel & 0x80) != 0{ //check if the 7th bit is active, ie signed, ie negative
            self.addr_rel |= 0xFF00;
        }
        0
    }
    fn ABS(&mut self)->u8{
        let lo:u16 = self.read(self.pc) as u16;
        self.pc += 1;
        let hi:u16 = self.read(self.pc) as u16;
        self.pc += 1;
        self.addr_abs = (hi<<8)|lo;
        0
    }
    fn ABX(&mut self)->u8{
        let lo:u16 = self.read(self.pc) as u16;
        self.pc += 1;
        let hi:u16 = self.read(self.pc) as u16;
        self.pc += 1;
        self.addr_abs = ((hi<<8 as u16)|lo) as u16;
        self.addr_abs += self.x as u16;
        if ( self.addr_abs & 0xff00) != (hi << 8) {
            return 1;
        }else{
            return 0;
        }
    }
    fn ABY(&mut self)->u8{
        let lo:u16 = self.read(self.pc) as u16;
        self.pc += 1;
        let hi:u16 = self.read(self.pc) as u16;
        self.pc += 1;
        self.addr_abs = ((hi<<8 as u16)|lo) as u16;
        self.addr_abs += self.y as u16;
        if ( self.addr_abs & 0xff00) != (hi << 8) {
            return 1;
        }else{
            return 0;
        }
    }
    fn IND(&mut self)->u8{
        let ptr_lo:u16 = self.read(self.pc) as u16;
        self.pc += 1;
        let ptr_hi:u16 = self.read(self.pc) as u16;
        self.pc += 1;

        let ptr:u16 = (ptr_hi << 8) | ptr_lo;

        if ptr_lo == 0x00FF // Simulate page boundary hardware bug
        {
            self.addr_abs = ((self.read(ptr & 0xFF00) as u16) << 8) | self.read(ptr + 0) as u16;
        }
        else // Behave normally
        {
            self.addr_abs = ((self.read(ptr + 1) as u16) << 8) | self.read(ptr + 0) as u16;
        }
        
        return 0; 

    }
    fn IZX(&mut self)->u8{
        let t:u16 = self.read(self.pc) as u16;
        self.pc += 1;
    
        let lo:u16 = self.read((t + (self.x as u16)) & 0x00FF) as u16;
        let hi:u16 = self.read((t + ((self.x + 1) as u16)) & 0x00FF) as u16;
    
        self.addr_abs = (hi << 8) | lo;
        
        return 0;
    }
    fn IZY(&mut self)->u8{
        let t:u16 = self.read(self.pc) as u16;
        self.pc += 1;
    
        let lo:u16 = self.read(t & 0x00FF) as u16;
        let hi:u16 = self.read((t + 1) & 0x00FF) as u16;
    
        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;
        
        if (self.addr_abs & 0xFF00) != (hi << 8){
            return 1;
        }else{
            return 0;
        }   
    }

//opcodes

    fn ADC(&mut self)->u8{
        self.fetch();
        let temp:u16 = self.a as u16 + self.fetched as u16 + self.get_flag(FLAGS6502::C) as u16;
        self.set_flag(FLAGS6502::C, temp > 255);
        self.set_flag(FLAGS6502::Z, (temp & 0x00ff) == 0);
        self.set_flag(FLAGS6502::N, (temp & 0x80) != 0);
        self.set_flag(FLAGS6502::V, ((!(self.a as u16 ^ self.fetched as u16) & (self.a as u16 ^ temp)) & 0x0080) != 0);
        self.a = (temp & 0x00ff) as u8;
        1
    }
    fn AND(&mut self)->u8{
        self.fetch();
        self.a &= self.fetched;
        self.set_flag(FLAGS6502::Z, self.a == 0x00);
        self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
        1
    }
    fn ASL(&mut self)->u8{
        self.fetch();
        let temp:u16 = (self.fetched as u16) << 1;
        self.set_flag(FLAGS6502::C, (temp & 0xFF00) > 0);
        self.set_flag(FLAGS6502::Z, (temp & 0x00FF) == 0x00);
        self.set_flag(FLAGS6502::N, (temp & 0x80 )!=0);
        if self.lookup[self.opcode as usize].addrmode as usize == P6502::IMP as usize{
            self.a = (temp & 0x00FF) as u8;
        }else{
            self.write(self.addr_abs, (temp & 0x00FF) as u8);
        }
        0
    }
    fn BCC(&mut self)->u8{
        if self.get_flag(FLAGS6502::C) == 0
        {
            self.cycles +=1;
            self.addr_abs = self.pc + self.addr_rel;
            
            if(self.addr_abs & 0xFF00) != (self.pc & 0xFF00){self.cycles +=1;}
            self.pc = self.addr_abs;
        }
        0
    }

    fn BCS(&mut self)->u8{
        if self.get_flag(FLAGS6502::C) == 0{
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xff00) != (self.pc & 0xff00){
                self.cycles +=1;
            }

            self.pc = self.addr_abs;
        }
        0
    }
    fn BEQ(&mut self)->u8{
        if self.get_flag(FLAGS6502::Z) == 1{
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xff00) != (self.pc & 0xff00){
                self.cycles +=1;
            }

            self.pc = self.addr_abs;
        }
        0
    }
    fn BIT(&mut self)->u8{
        self.fetch();
        let temp = self.a & self.fetched;
        self.set_flag(FLAGS6502::Z, (temp & 0x00FF) == 0x00);
        self.set_flag(FLAGS6502::N, (self.fetched & (1 << 7)) != 0);
        self.set_flag(FLAGS6502::V, (self.fetched & (1 << 6)) != 0);
        return 0;
    }
    fn BMI(&mut self)->u8{
        if self.get_flag(FLAGS6502::N) == 1{
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xff00) != (self.pc & 0xff00){
                self.cycles +=1;
            }

            self.pc = self.addr_abs;
        }
        0
    }

    fn BNE(&mut self)->u8{
        if self.get_flag(FLAGS6502::Z) == 0{
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xff00) != (self.pc & 0xff00){
                self.cycles +=1;
            }

            self.pc = self.addr_abs;
        }
        0
    }
    fn BPL(&mut self)->u8{
        if self.get_flag(FLAGS6502::N) == 0{
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xff00) != (self.pc & 0xff00){
                self.cycles +=1;
            }

            self.pc = self.addr_abs;
        }
        0
    }
    fn BRK(&mut self)->u8{
        self.pc += 1;
	
        self.set_flag(FLAGS6502::I, true);
        self.write(0x0100 + self.stkp as u16, ((self.pc >> 8) & 0x00FF) as u8 ) ;
        self.stkp -= 1;
        self.write(0x0100 + self.stkp as u16, self.pc as u8 & 0x00FF);
        self.stkp -= 1;
    
        self.set_flag(FLAGS6502::B, true);
        self.write(0x0100 + self.stkp as u16, self.status);
        self.stkp -= 1;
        self.set_flag(FLAGS6502::B, false);
    
        self.pc = self.read(0xFFFE) as u16 | (self.read(0xFFFF) as u16) << 8;
        0
    }
    fn BVC(&mut self)->u8{
        if self.get_flag(FLAGS6502::V) == 0{
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xff00) != (self.pc & 0xff00){
                self.cycles +=1;
            }

            self.pc = self.addr_abs;
        }
        0
    }

    fn BVS(&mut self)->u8{
        if self.get_flag(FLAGS6502::V) == 1{
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xff00) != (self.pc & 0xff00){
                self.cycles +=1;
            }

            self.pc = self.addr_abs;
        }
        0
    }
    fn CLC(&mut self)->u8{
        self.set_flag(FLAGS6502::C, false);
        0
    }
    fn CLD(&mut self)->u8{
        self.set_flag(FLAGS6502::D, false);
        0
    }
    fn CLI(&mut self)->u8{
        self.set_flag(FLAGS6502::I, false);
        0
    }

    fn CLV(&mut self)->u8{
        self.set_flag(FLAGS6502::V, false);
        0
    }
    fn CMP(&mut self)->u8{
        self.fetch();
        let temp = self.a as u16 - self.fetched as u16;
        self.set_flag(FLAGS6502::C, self.a >= self.fetched);
        self.set_flag(FLAGS6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(FLAGS6502::N, ( temp & 0x0080 ) != 0);
        1
    }
    fn CPX(&mut self)->u8{
        self.fetch();
        let temp = self.a as u16 - self.fetched as u16;
        self.set_flag(FLAGS6502::C, self.x >= self.fetched);
        self.set_flag(FLAGS6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(FLAGS6502::N, ( temp & 0x0080 ) != 0);
        1
    }
    fn CPY(&mut self)->u8{
        self.fetch();
        let temp = self.a as u16 - self.fetched as u16;
        self.set_flag(FLAGS6502::C, self.y >= self.fetched);
        self.set_flag(FLAGS6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(FLAGS6502::N, ( temp & 0x0080 ) != 0);
        1
    }

    fn DEC(&mut self)->u8{
        self.fetch();
        let temp = self.fetched - 1;
        self.write(self.addr_abs, temp & 0x00FF);
        self.set_flag(FLAGS6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(FLAGS6502::N, (temp & 0x0080) != 0 );
        0
    }
    fn DEX(&mut self)->u8{
        self.x -= 1;
        self.set_flag(FLAGS6502::Z, self.x == 0x00);
        self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
        0
    }
    fn DEY(&mut self)->u8{
        self.y -= 1;
        self.set_flag(FLAGS6502::Z, self.y == 0x00);
        self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
        0
    }
    fn EOR(&mut self)->u8{
        self.fetch();
        self.a = self.a ^ self.fetched;	
        self.set_flag(FLAGS6502::Z, self.a == 0x00);
        self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
        1
    }

    fn INC(&mut self)->u8{
        self.fetch();
        let temp = self.fetched + 1;
        self.write(self.addr_abs, temp & 0x00FF);
        self.set_flag(FLAGS6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(FLAGS6502::N, (temp & 0x0080) != 0);
        0
    }
    fn INX(&mut self)->u8{
        self.x += 1;
        self.set_flag(FLAGS6502::Z, self.x == 0x00);
        self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
        0
    }
    fn INY(&mut self)->u8{
        self.y += 1;
        self.set_flag(FLAGS6502::Z, self.y == 0x00);
        self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
        0
    }
    fn JMP(&mut self)->u8{
        self.pc = self.addr_abs;
        0
    }

    fn JSR(&mut self)->u8{
        self.pc -= 1;

        self.write(0x0100 + self.stkp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.stkp -=1;
        self.write(0x0100 + self.stkp as u16, (self.pc & 0x00FF) as u8);
        self.stkp -=1;
    
        self.pc = self.addr_abs;
        0
    }
    fn LDA(&mut self)->u8{
        self.fetch();
        self.a = self.fetched;
        self.set_flag(FLAGS6502::Z, self.a == 0x00);
        self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
        1
    }
    fn LDX(&mut self)->u8{
        self.fetch();
        self.x = self.fetched;
        self.set_flag(FLAGS6502::Z, self.x == 0x00);
        self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
        1
    }
    fn LDY(&mut self)->u8{
        self.fetch();
        self.y = self.fetched;
        self.set_flag(FLAGS6502::Z, self.y == 0x00);
        self.set_flag(FLAGS6502::N, (self.y & 0x80) != 0);
        1
    }

    fn LSR(&mut self)->u8{
        self.fetch();
        self.set_flag(FLAGS6502::C, (self.fetched & 0x0001) != 0);
        let temp = self.fetched >> 1;	
        self.set_flag(FLAGS6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(FLAGS6502::N, (temp & 0x0080) != 0);
        if self.lookup[self.opcode as usize].addrmode as usize == P6502::IMP as usize{
            self.a = temp & 0x00FF;
        }else{
            self.write(self.addr_abs, temp & 0x00FF);
        }
        0
    }
    fn NOP(&mut self)->u8{
        match self.opcode{
            0x1C|0x3C|0x5C|0x7C|0xDC|0xFC => 1,
            _ => 0,
        }
    }
    fn ORA(&mut self)->u8{
        self.fetch();
        self.a = self.a | self.fetched;
        self.set_flag(FLAGS6502::Z, self.a == 0x00);
        self.set_flag(FLAGS6502::N, ( self.a & 0x80) != 0);
        1
    }
    fn PHA(&mut self)->u8{
        self.write(0x0100 + self.stkp as u16, self.a);
        self.stkp -= 1;
        0
    }

    fn PHP(&mut self)->u8{
        self.write(0x0100 + self.stkp as u16, ( self.status | FLAGS6502::B as u8 | FLAGS6502::U as u8) as u8);
        self.set_flag(FLAGS6502::B, false);
        self.set_flag(FLAGS6502::U, false);
        self.stkp -= 1;
        0
    }
    fn PLA(&mut self)->u8{
        self.stkp += 1;
        self.a = self.read(0x0100 + self.stkp as u16);
        self.set_flag(FLAGS6502::Z, self.a == 0x00);
        self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
        0
    }
    fn PLP(&mut self)->u8{
        self.stkp += 1;
        self.status = self.read(0x0100 + self.stkp as u16);
        self.set_flag(FLAGS6502::U, true);
        0
    }
    fn ROL(&mut self)->u8{
        self.fetch();
        let temp:u16 = ((self.fetched << 1) | self.get_flag(FLAGS6502::C)) as u16;
        self.set_flag(FLAGS6502::C, (temp & 0xFF00) != 0);
        self.set_flag(FLAGS6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(FLAGS6502::N, (temp & 0x0080) != 0);
        if self.lookup[self.opcode as usize].addrmode as usize == P6502::IMP as usize{
            self.a = temp as u8 & 0x00FF;
        }else{
            self.write(self.addr_abs, temp as u8 & 0x00FF);
        }
        0
    }

    fn ROR(&mut self)->u8{
        self.fetch();
        let temp:u16 = ((self.fetched >> 1) | self.get_flag(FLAGS6502::C) << 7) as u16;
        self.set_flag(FLAGS6502::C, (self.fetched & 0x01) != 0);
        self.set_flag(FLAGS6502::Z, (temp & 0x00FF) == 0x0000);
        self.set_flag(FLAGS6502::N, (temp & 0x0080) != 0);
        if self.lookup[self.opcode as usize].addrmode as usize == P6502::IMP as usize{
            self.a = temp as u8 & 0x00FF;
        }else{
            self.write(self.addr_abs, temp as u8 & 0x00FF);
        }
        0
    }
    fn RTI(&mut self)->u8{
        self.stkp += 1;
        self.status = self.read(0x0100 + self.stkp as u16);
        self.status &= !(FLAGS6502::B as u8);
        self.status &= !(FLAGS6502::U as u8);

        self.stkp += 1;
        self.pc = self.read(0x0100 + self.stkp as u16) as u16;
        self.stkp += 1;
        self.pc |= (self.read(0x0100 + self.stkp as u16) as u16) << 8;

        0
    }
    fn RTS(&mut self)->u8{
        self.stkp += 1;
        self.pc = self.read(0x0100 + self.stkp as u16) as u16;
        self.stkp += 1;
        self.pc |= (self.read(0x0100 + self.stkp as u16) as u16) << 8;
        
        self.pc += 1;
        0
    }
    fn SBC(&mut self)->u8{
        let value:u16 = self.fetched as u16 ^ 0x00FF;
        let temp: u16 = self.a as u16 + value + self.get_flag(FLAGS6502::C) as u16;
        self.set_flag(FLAGS6502::C, (temp & 0xFF00) != 0);
        self.set_flag(FLAGS6502::Z, (temp & 0x00FF) == 0);
        self.set_flag(FLAGS6502::V, (temp ^ (self.a as u16) & (temp ^ value) & 0x0080) != 0);
        self.set_flag(FLAGS6502::N, ( temp & 0x0080 ) != 0);
        self.a = (temp & 0x00FF) as u8;
        1
    }

    fn SEC(&mut self)->u8{
        self.set_flag(FLAGS6502::C, true);
        0
    }
    fn SED(&mut self)->u8{
        self.set_flag(FLAGS6502::D, true);
        0
    }
    fn SEI(&mut self)->u8{
        self.set_flag(FLAGS6502::I, true);
        0
    }
    fn STA(&mut self)->u8{
        self.write(self.addr_abs, self.a);
        0
    }

    fn STX(&mut self)->u8{
        self.write(self.addr_abs, self.x);
        0
    }
    fn STY(&mut self)->u8{
        self.write(self.addr_abs, self.y);
        0
    }
    fn TAX(&mut self)->u8{
        self.x = self.a;
        self.set_flag(FLAGS6502::Z, self.x == 0x00);
        self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
        0
    }
    fn TAY(&mut self)->u8{
        self.y = self.a;
        self.set_flag(FLAGS6502::Z, self.x == 0x00);
        self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
        0
    }

    fn TSX(&mut self)->u8{
        self.x = self.stkp;
        self.set_flag(FLAGS6502::Z, self.x == 0x00);
        self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
        0
    }
    fn TXA(&mut self)->u8{
        self.a = self.x;
        self.set_flag(FLAGS6502::Z, self.x == 0x00);
        self.set_flag(FLAGS6502::N, (self.x & 0x80) != 0);
        0
    }
    fn TXS(&mut self)->u8{
        self.stkp = self.x;
        0
    }
    fn TYA(&mut self)->u8{
        self.a = self.y;
        self.set_flag(FLAGS6502::Z, self.a == 0x00);
        self.set_flag(FLAGS6502::N, (self.a & 0x80) != 0);
        0
    }

    fn XXX(&mut self)->u8{0}

//cpu functions
    fn reset(&mut self){
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0xfd;
        self.status = 0x00 | FLAGS6502::U as u8;

        self.addr_abs = 0xfffc;
        let lo:u16 = self.read(self.addr_abs+0) as u16;
        let hi:u16 = self.read(self.addr_abs+1) as u16;

        self.pc = (hi <<8)|lo;

        self.addr_abs = 0x0000;
        self.addr_rel = 0x0000;
        self.fetched = 0x00;

        self.cycles = 8;

    }
    fn irq(&mut self){
        if self.get_flag(FLAGS6502::I) == 0{
            self.write(0x0100 + self.stkp as u16, (self.pc >> 8) as u8 & 0x00ff);
            self.stkp -= 1;
            self.write(0x0100 + self.stkp as u16, self.pc as u8 & 0x00ff);
            self.stkp -= 1;

            self.set_flag(FLAGS6502::B, false);
            self.set_flag(FLAGS6502::U, true);
            self.set_flag(FLAGS6502::I, true);
            self.write(0x0100 + self.stkp as u16, self.status);
            self.stkp -= 1;

            self.addr_abs = 0xfffe;
            let lo:u16 = self.read(self.addr_abs+0) as u16;
            let hi:u16 = self.read(self.addr_abs+1) as u16;
            self.pc = (hi <<8)|lo;

            self.cycles = 7;

        }
    }
    fn nmi(&mut self){
        self.write(0x0100 + self.stkp as u16, (self.pc >> 8) as u8 & 0x00ff);
        self.stkp -= 1;
        self.write(0x0100 + self.stkp as u16, self.pc as u8 & 0x00ff);
        self.stkp -= 1;

        self.set_flag(FLAGS6502::B, false);
        self.set_flag(FLAGS6502::U, true);
        self.set_flag(FLAGS6502::I, true);
        self.write(0x0100 + self.stkp as u16, self.status);
        self.stkp -= 1;

        self.addr_abs = 0xfffe;
        let lo:u16 = self.read(self.addr_abs+0) as u16;
        let hi:u16 = self.read(self.addr_abs+1) as u16;
        self.pc = (hi <<8)|lo;

        self.cycles = 7;
    }


//memory operations

    fn fetch(&mut self)->u8{
        if !(self.lookup[self.opcode as usize].addrmode as usize == P6502::IMP as usize){
            self.fetched = self.read(self.addr_abs);
        }
        return self.fetched;
    }
    pub fn read(&self, a: u16) -> u8{
        self.bus.read(a, false)
    }

    pub fn write(&mut self, a:u16, d:u8){
        self.bus.write(a, d)        
    }

    fn clock(&mut self){
        if self.cycles == 0 as u8{
            self.opcode = self.read(self.pc);
            self.pc += 1;

            self.cycles = self.lookup[self.opcode as usize].cycles;

            let addrmode= self.lookup[self.opcode as usize].addrmode;
            let operate = self.lookup[self.opcode as usize].operate;

            let additional_cycle1 = addrmode(self);
            let additional_cycle2 = operate(self);

            self.cycles += additional_cycle1 & additional_cycle2;
        }

        self.cycles -= 1;

    }

//flag operations
    pub fn get_flag(&self, f:FLAGS6502)->u8{
        match f {
            FLAGS6502::C => self.status & 0b0000_0001,
            FLAGS6502::Z => self.status & 0b0000_0010,
            FLAGS6502::I => self.status & 0b0000_0100,
            FLAGS6502::D => self.status & 0b0000_1000,
            FLAGS6502::B => self.status & 0b0001_0000,
            FLAGS6502::U => self.status & 0b0010_0000,
            FLAGS6502::V => self.status & 0b0100_0000,
            FLAGS6502::N => self.status & 0b1000_0000,
        }   
    }

    pub fn set_flag(&mut self, f:FLAGS6502, v:bool){
        match f {
            //FLAGS6502::C => self.status |= 0b0000_0001,
            FLAGS6502::C => self.status |= ( v as u8 ) << 0,
            FLAGS6502::Z => self.status |= ( v as u8 ) << 1,
            FLAGS6502::I => self.status |= ( v as u8 ) << 2,
            FLAGS6502::D => self.status |= ( v as u8 ) << 3,
            FLAGS6502::B => self.status |= ( v as u8 ) << 4,
            FLAGS6502::U => self.status |= ( v as u8 ) << 5,
            FLAGS6502::V => self.status |= ( v as u8 ) << 6,
            FLAGS6502::N => self.status |= ( v as u8 ) << 7,            
        }

    }
}
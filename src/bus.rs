//mod mem;
//mod cpu;
use crate::olc6502::OLC6502;

const MAX_MEM: usize = 1024 * 64;

pub struct BUS{
    //pub cpu:CPU,
    pub ram:[u8;MAX_MEM],
}

impl BUS{
    pub fn new ()->BUS{
        BUS{
            ram:[0;MAX_MEM]
        }
        // CPU{pc :0xFFFC,
        // sp : 0x0100,
        // a: 0,
        // x: 0,
        // y: 0,
        // flags: 0b0000_0000}        
    }    
    pub fn write(&mut self, addr: u16, data:u8){
        if addr >= 0x0000 && addr <= 0xFFFF{
            self.ram[addr as usize] = data;
        }
    }
    pub fn read(&self, addr:u16, b_read_only: bool) -> u8{
        if addr >= 0x0000 && addr <= 0xFFFF{
            return self.ram[addr as usize];
        }
        return 0x00;
    }
}
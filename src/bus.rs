//mod mem;
//mod cpu;
//use crate::P6502::P6502;

const MAX_MEM: usize = 1024 * 64;

pub struct BUS{
    //pub cpu:CPU,
    pub ram:[u8;MAX_MEM],
}

impl BUS{
    pub fn new ()->BUS{
        BUS{
            ram:[1;MAX_MEM]
        }      
    }    
    pub fn write(&mut self, addr: u16, data:u8){
        #[allow(unused_comparisons)]
        if addr >= 0x0000 && addr <= 0xFFFF{
            self.ram[addr as usize] = data;
        }
    }
    pub fn read(&self, addr:u16, _b_read_only: bool) -> u8{
        #[allow(unused_comparisons)]
        if addr >= 0x0000 && addr <= 0xFFFF{
            return self.ram[addr as usize];
        }
        return 0x00;
    }
}
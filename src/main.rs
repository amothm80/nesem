//use std;
mod p6502;
mod bus;
use crate::p6502::P6502;
//use crate::bus::BUS;

fn main() {
    println!("Hello, 1world!");
    #[allow(unused_variables)]
    let cpu = P6502::new();
    println!("Hello, 1world!");
}

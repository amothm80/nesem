use std;
mod olc6502;
mod bus;
use crate::olc6502::OLC6502;
use crate::bus::BUS;

fn main() {
    println!("Hello, world!");
    let mut cpu = OLC6502::new();
}

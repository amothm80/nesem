mod p6502;
mod bus;
use crate::p6502::P6502;
use console_engine::ConsoleEngine;
use console_engine::pixel;
use console_engine::Color;
use console_engine::KeyCode;

struct ram{data:u16}

fn main() {
    #[allow(unused_variables)]
    let mut cpu = P6502::new();
    // initializes a screen of 20x10 characters with a target of 3 frames per second
    // coordinates will range from [0,0] to [19,9]
    let mut engine = console_engine::ConsoleEngine::init(150, 40, 15).unwrap();
    //let value = 14;
    //let mut r = ram{data : 100};
    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        draw_ram(&mut engine, &cpu,  0, 0, 0, 16, 16);
        draw_ram(&mut engine, &cpu, 0, 17, 0x8000, 16, 16);
        draw_cpu(&mut engine, &cpu, 60, 0, 0, 0);
        if engine.is_key_pressed(KeyCode::Char('q')) { // if the user presses 'q' :
            break; // exits app
        }
        // if engine.is_key_pressed(KeyCode::Char('n')){
        //     cpu.status = 0b1000_0000;
        // }
        // if engine.is_key_pressed(KeyCode::Char('-')){
        //     r.data -= 1;
        // }        
        engine.draw(); // draw the screen
        //println!("{:04x}", 277);
    }
}



fn draw_ram(engine:&mut ConsoleEngine, cpu:&p6502::P6502, x:u8, y:u8, addr:u16, rows:u16, cols:u16){
    let nramx = x;
    let mut nramy = y;
    
    let mut naddr = addr;
    for row in 0..rows{
        let mut s_offset = String::from("");
        s_offset.push('$');
        s_offset.push_str(format!("{:04x}", naddr).as_str());
        s_offset.push(':');
        for col in 0..cols{
            s_offset.push(' ');
            s_offset.push_str(format!("{:02x}", cpu.bus.read(naddr, true)).as_str() /*todo add read address from nes memory here */);
            naddr += 1;
        }
        engine.print(nramx as i32, nramy as i32, &s_offset);
        nramy += 1;
    }
}

fn draw_cpu(engine:&mut ConsoleEngine, cpu:&p6502::P6502, x:u8, y:u8, rows:u16, cols:u16){
    let mut s_offset = String::from("STATUS: ");
    engine.print_fbg(x as i32, y as i32, s_offset.as_str(), Color::White, Color::Black);
    engine.print_fbg(x as i32 + 8 , y as i32, "N", if (cpu.status & p6502::FLAGS6502::N as u8) != 0 {Color::Green}else{Color::Red} , Color::Black);
    engine.print_fbg(x as i32 + 10 , y as i32, "V", if (cpu.status & p6502::FLAGS6502::V as u8) != 0 {Color::Green}else{Color::Red} , Color::Black);
    engine.print_fbg(x as i32 + 12 , y as i32, "U", if (cpu.status & p6502::FLAGS6502::U as u8) != 0 {Color::Green}else{Color::Red} , Color::Black);
    engine.print_fbg(x as i32 + 14 , y as i32, "B", if (cpu.status & p6502::FLAGS6502::B as u8) != 0 {Color::Green}else{Color::Red} , Color::Black);
    engine.print_fbg(x as i32 + 16 , y as i32, "D", if (cpu.status & p6502::FLAGS6502::D as u8) != 0 {Color::Green}else{Color::Red} , Color::Black);
    engine.print_fbg(x as i32 + 18 , y as i32, "I", if (cpu.status & p6502::FLAGS6502::I as u8) != 0 {Color::Green}else{Color::Red} , Color::Black);
    engine.print_fbg(x as i32 + 20 , y as i32, "Z", if (cpu.status & p6502::FLAGS6502::Z as u8) != 0 {Color::Green}else{Color::Red} , Color::Black);
    engine.print_fbg(x as i32 + 22 , y as i32, "C", if (cpu.status & p6502::FLAGS6502::C as u8) != 0 {Color::Green}else{Color::Red} , Color::Black);

    // DrawString(x , y + 10, "PC: $" + hex(nes.cpu.pc, 4));
    // DrawString(x , y + 20, "A: $" +  hex(nes.cpu.a, 2) + "  [" + std::to_string(nes.cpu.a) + "]");
    // DrawString(x , y + 30, "X: $" +  hex(nes.cpu.x, 2) + "  [" + std::to_string(nes.cpu.x) + "]");
    // DrawString(x , y + 40, "Y: $" +  hex(nes.cpu.y, 2) + "  [" + std::to_string(nes.cpu.y) + "]");
    // DrawString(x , y + 50, "Stack P: $" + hex(nes.cpu.stkp, 4));
}

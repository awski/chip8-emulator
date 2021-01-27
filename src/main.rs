mod chip8;

use chip8::Chip8;
use std::env;

// TODO(#3): timers & sound

// TODO(#5): keyboard mapping

// TODO(#7): ROM selector
// provide rom path as an arg or interactive stdin

// TODO(#11): CPU tests

// TODO(#17): SDL2(or similar) gfx display

// TODO(#18): rework unit tests
fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut ch8 = Chip8::new();
    ch8.load_rom("roms/1dcell.ch8");
    //println!("{:?}", em.ram);
    //let word = em.cpu.read_instr(em.ram);
    //em.cpu.exec_instr(word);

    match args.len() {
        2 => {
            match args[1].as_str() {
                "dbg" => { ch8.dbg() },
                _ => { ch8.run() }
            }
        }
        _ => {
            ch8.run();
        }
    }
}

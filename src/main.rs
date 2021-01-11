mod chip8;
use chip8 as emulator;

// TODO(#3): timers & sound

// TODO(#5): keyboard mapping

// TODO(#6): 64x32 display

// TODO(#7): ROM selector
// provide rom path as an arg or interactive stdin
fn main() {
    let mut em = emulator::Chip8::new();
    em.load_rom("roms/1dcell.ch8");
    println!("{:?}", em.ram);
}

// fn decode(op: u16) {

//     let w = op & 0xF000 >> 12;
//     let x = op & 0x0F00 >> 8;
//     let y = op & 0x00F0 >> 4;
//     let z = op & 0x000F >> 0;

//     match (op & 0xF000, op & 0x0F00, op & 0x00F0, op & 0x000F) {
//         (0, 0, y, 0) => { println!("CLS"); }
//         (0, 0, y, z) => { println!("RET"); }
//         (w, _, _, _) => { println!("DRW Vx, Vy, nibble");  }
//         (_, _, _, _) => { println!("OPCode unknown found: {:x}", op); }
//     }
// }

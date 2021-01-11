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
    //println!("{:?}", em.ram);
    let word = em.cpu.read_instr(em.ram);
    em.cpu.exec_instr(word);
}
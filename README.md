# Chip8 emulator (in-development)

Simple emulator for .ch8 files written in Rust

### TODOs

- [ ] 64x32 display
- [ ] Keyboard mapping
- [ ] Super ch8 extension
- [ ] Interactive ROM selection
- [ ] Debugger

### Usage example
```rust
mod chip8;
use chip8 as emulator;

fn main() {
    let mut em = emulator::Chip8::new();
    em.load_rom("roms/1dcell.ch8");
    em.ram.dbg_print_memory(10);
}
```
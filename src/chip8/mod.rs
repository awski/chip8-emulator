mod ram;
mod cpu;

use cpu::Cpu;
use ram::Ram;

pub struct Chip8 {
    pub ram: Ram,
    pub cpu: Cpu,
}
impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            ram: Ram::new(),
            cpu: Cpu::new(),
        }
    }
    pub fn load_rom(&mut self, path: &str) {
        let data = match std::fs::read(path) {
            Ok(data) => { data },
            Err(err) => { panic!("file not found, ret{:?}!", err) },
        };
        for i in 0..data.len() {
            self.ram.write(i + ram::RAM_START_OFFSET, data[i]);
        }
    }
}

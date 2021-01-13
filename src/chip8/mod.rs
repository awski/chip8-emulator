#[cfg(test)]
mod tests;
mod ram;
mod cpu;

use std::io::{self, Read};

use cpu::Cpu;
use ram::Ram;

pub struct Chip8 {
    ram: Ram,
    cpu: Cpu,
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
            self.ram.write(i + ram::RAM_START_OFFSET as usize, data[i]);
        }
    }
    pub fn run(&mut self) {
        loop {
            let op_code = self.cpu.read_instr(&self.ram);
            self.cpu.exec_instr(&mut self.ram, op_code);
        }
    }

    pub fn dbg(&mut self) {
        loop {
            let op_code = self.cpu.read_instr(&self.ram);
            io::stdin().read(&mut [0]).unwrap();
            self.cpu.exec_instr(&mut self.ram, op_code);
        }
    }
}

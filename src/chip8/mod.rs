#[cfg(test)]
mod tests;
mod ram;
mod cpu;
mod display;

use std::io::{self, Read};
use cpu::Cpu;
use ram::Ram;
use display::Display;

pub struct Chip8 {
    ram: Ram,
    display: Display,
    cpu: Cpu,
}
impl Chip8 {
    pub fn new() -> Chip8 {
        let mut ch8 = Chip8 {
            ram: Ram::new(),
            display: Display::new(),
            cpu: Cpu::new(),
        };

        Display::load_fonts(&mut ch8.ram);

        return ch8
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
            self.cpu.exec_instr(&mut self.ram, &mut self.display, op_code);
            println!("{:?}", self.display);
        }
    }

    pub fn dbg(&mut self) {
        loop {
            io::stdin().read(&mut [0]).unwrap();
            println!("{:?}", self.cpu);
            println!("{:?}", self.display);
            let op_code = self.cpu.read_instr(&self.ram);
            self.cpu.exec_instr(&mut self.ram, &mut self.display, op_code);
        }
    }
}

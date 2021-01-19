use std::fmt;

const MEMORY_SIZE: usize = 4096;
const RAM_END_OFFSET: u16 = 0xFFF;
const _RAM_ETI_START_OFFSET: u16 = 0x600;
pub const RAM_START_OFFSET: u16 = 0x200;

pub struct Ram {
    pub memory: [u8;MEMORY_SIZE],
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            memory: [0; MEMORY_SIZE],
        }
    }
    pub fn read(&self, idx: usize) -> u8 {
        assert!(idx <= RAM_END_OFFSET as usize);
        self.memory[idx]
    }
    pub fn write(&mut self, idx: usize, value: u8) {
        assert!(idx <= RAM_END_OFFSET as usize);
        self.memory[idx] = value;
    }
}

impl fmt::Debug for Ram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.memory.len() {
            if i % 8 == 0 {
                write!(f, "{:<5x}|", i)?;
            }
            write!(f, "{:<5x}", self.memory[i])?;
            if i % 8 == 8 - 1 {
                write!(f, "\n")?;
            }
        }
        write!(f, "")
    }
}
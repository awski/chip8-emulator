const MEMORY_SIZE: usize = 4096;
const RAM_END_OFFSET: usize = 0xFFF;
const RAM_ETI_START_OFFSET: usize = 0x600;
pub const RAM_START_OFFSET: usize = 0x200;

pub struct Ram {
    memory: [u8;MEMORY_SIZE],
}

impl Ram {
    pub (in crate::chip8) fn new() -> Ram{
        Ram {
            memory: [0; MEMORY_SIZE],
        }
    }
    pub (in crate::chip8) fn read(&self, idx: usize) -> u8 {
        assert!(idx <= RAM_END_OFFSET);
        self.memory[idx]
    }
    pub (in crate::chip8) fn write(&mut self, idx: usize, value: u8) {
        assert!(idx <= RAM_END_OFFSET);
        self.memory[idx] = value;
    }

    pub fn dbg_print_memory(&mut self, row_size: usize) {
        for i in 0..self.memory.len() {
            if i % row_size == 0 {
                print!("{:<5x}|", i);
            }
            print!("{:<5x}", self.memory[i]);
            if i % row_size == row_size - 1 {
                print!("\n");
            }
        }
    }
}
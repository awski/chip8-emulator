// TODO(#1): CPU implementation
// registers, pointers

// TODO: chip8 instructions

// TODO: super chip8 instruction
// an extension of standard chip8 instruction

pub struct Cpu {
    reg_vx: [u8; 16],
    index_reg: u16,
    program_cnt: u16,
    stack: [u16; 16],
    stack_ptr: u8,
}

impl Cpu {
    pub (in crate::chip8) fn new() -> Cpu {
        Cpu {
            reg_vx: [0; 16],
            i: 0,
            pc: 0,
        }
    }

    #[allow(dead_code)]
    pub (in crate::chip8) fn print_vx(&self) {
        for (i, element) in self.reg_vx.iter().enumerate() {
            println!("{}. - {}", i, element);
        }
    }

    #[allow(dead_code)]
    fn set_vx(&mut self, reg_idx: u8, value: u8) {
        self.reg_vx[reg_idx as usize] = value;
    }

    #[allow(dead_code)]
    fn get_vx(&self, reg_idx: u8) -> u8 {
        self.reg_vx[reg_idx as usize]
    }
}

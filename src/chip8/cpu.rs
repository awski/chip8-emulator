// TODO: CPU implementation
// registers, pointers

// TODO: chip8 instructions

// TODO: super chip8 instruction
// an extension of standard chip8 instruction

pub struct Cpu {
    reg_vx: [u8; 16],
}
impl Cpu {
    pub (in crate::chip8) fn new() -> Cpu {
        Cpu {
            reg_vx: [0; 16],
        }
    }
    #[allow(dead_code)]
    pub (in crate::chip8) fn print_vx(&self) {
        for (i, element) in self.reg_vx.iter().enumerate() {
            println!("{}. - {}", i, element);
        }
    }
}

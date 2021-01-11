// TODO(#4): super chip8 instruction
// an extension of standard chip8 instruction

use super::ram;

pub struct Cpu {
    reg_vx: [u8; 16],
    reg_idx: u16,
    prog_cnt: u16,
    stack: [u16; 16],
    stack_ptr: u8,
}

impl Cpu {
    pub (in crate::chip8) fn new() -> Cpu {
        Cpu {
            reg_vx:     [0; 16],
            reg_idx:    0,
            prog_cnt:   ram::RAM_START_OFFSET,
            stack:      [0; 16],
            stack_ptr:  0,
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

    pub fn read_instr(&self, ram: ram::Ram) -> u16{
        let hi = ram.read(self.prog_cnt as usize) as u16;
        let lo = ram.read(1 + self.prog_cnt as usize) as u16;

        u16::from_be(hi | (lo << 8))
        //println!("hi: 0x{:04x}, lo: 0x{:04x?}, instr: 0x{:X?}", hi,lo,instr);
    }

    // TODO(#2): chip8 instructions
    pub fn exec_instr(&self, op: u16) {
        let w = op & 0xF000 >> 12;
        let x = op & 0x0F00 >> 8;
        let y = op & 0x00F0 >> 4;
        let z = op & 0x000F >> 0;

        match (op & 0xF000, op & 0x0F00, op & 0x00F0, op & 0x000F) {
            (0, 0, y, 0) => { println!("CLS"); }
            (0, 0, y, z) => { println!("RET"); }
            (w, _, _, _) => { println!("DRW Vx, Vy, nibble"); }
            (_, _, _, _) => { println!("OPCode unknown found: {:x}", op); }
        }
    }
}
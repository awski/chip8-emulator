// TODO(#4): super chip8 instruction
// an extension of standard chip8 instruction
use std::fmt;
use super::ram;

pub struct Cpu {
    reg_vx: [u8; 16],
    reg_idx: u16,
    prog_cnt: u16,
    stack: [u16; 16],
    stack_ptr: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg_vx:     [0; 16],
            reg_idx:    0,
            prog_cnt:   ram::RAM_START_OFFSET,
            stack:      [0; 16],
            stack_ptr:  0,
        }
    }

    pub fn read_instr(&self, ram: &ram::Ram) -> u16{
        let hi = ram.read(self.prog_cnt as usize) as u16;
        let lo = ram.read(1 + self.prog_cnt as usize) as u16;

        lo | (hi << 8)
    }

    // TODO(#10): refactor instructions
    pub fn exec_instr(&mut self, ram: &mut ram::Ram, op_code: u16) {
        let addr = op_code & 0x0FFF as u16;
        let nibble = (op_code & 0x000F) as u8;
        let byte = (op_code & 0x00FF) as u8;
        let x = ((op_code & 0x0F00) >> 8) as u8;
        let y = ((op_code & 0x00F0) >> 4) as u8;

        let op_tup = (
            ((op_code & 0xF000) >> 12),
            ((op_code & 0x0F00) >> 8),
            ((op_code & 0x00F0) >> 4),
            ((op_code & 0x000F) >> 0), 
        );

        match op_tup {
            (0x0, 0x0, 0xE, 0x0) => {
                println!("not impl: CLR");
                self.prog_cnt += 2;
            }
            (0x0, 0x0, 0xE, 0xE) => {
                println!("RET");
                self.prog_cnt = self.stack_pop();
            }
            (0x1, _, _, _) => {
                println!("JMP to addr 0x{:04x?}", addr);
                self.prog_cnt = addr;
            },
            (0x2, _, _, _) => {
                println!("Call subroutine at 0x{:04x?}", addr);
                self.stack_push(self.prog_cnt);
                self.prog_cnt = addr;
            }
            (0x6, _, _, _) => {
                println!("Set V{:x?} = {:02x?}", x, byte);
                self.reg_vx[x as usize] = byte;
                self.prog_cnt += 2;
            }
            (0x7, _, _, _) => {
                println!("Set V{:x?} = V{:x?} + {:02x?}", x, x, byte);
                self.reg_vx[x as usize] = self.reg_vx[x as usize] + byte;
                self.prog_cnt += 2;
            }
            (0x8, _, _, 0x0) => {
                debug_assert!(x != y);

                println!("Set V{:x?} = V{:x?}", x, y);
                self.reg_vx[x as usize] = self.reg_vx[y as usize];
                self.prog_cnt += 2;
            }
            (0xA, _, _, _) => {
                println!("Set I = {:04x?}", addr);
                self.reg_idx = addr;
                self.prog_cnt += 2;
            }
            (0xD, _, _, _) => {
                println!("not impl: Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.");
                self.prog_cnt += 2;
            }
            (0xF, _, 0x2, 0x9) => {
                println!("not impl: Set I = location of sprite for digit Vx.");
                self.prog_cnt += 2;
            }
            (0xF, _, 0x3, 0x3) => {
                println!("Store BCD representation of Vx in memory locations I, I+1, and I+2.");
                ram.write(self.reg_idx as usize, self.reg_vx[x as usize] / 100);
                ram.write((self.reg_idx as usize) + 1, (self.reg_vx[x as usize] % 100) /10);
                ram.write((self.reg_idx as usize) + 2, self.reg_vx[x as usize] % 10);
                self.prog_cnt += 2;
            }
            (0xF, _, 0x6, 0x5) => {
                println!("Read registers V0 through Vx from memory starting at location I.");
                for i in 0..self.reg_vx.len() {
                    //ram.write((self.reg_idx + i as u16) as usize, self.reg_vx[i]);
                    self.reg_vx[i] = ram.read((self.reg_idx + i as u16) as usize);
                }
                self.prog_cnt += 2;
            }
            _ => { panic!("OPCode unknown found: 0x{:04x?} tup: {:?}", op_code, op_tup); }
        }
    }
}

impl Cpu { //Internals
    fn stack_push(&mut self, value: u16) {
        self.stack[self.stack_ptr as usize] = value;
        self.stack_ptr += 1;
    }

    fn stack_pop(&mut self) -> u16 {
        self.stack_ptr -= 1;
        self.stack[self.stack_ptr as usize]
    }

    fn _set_vx(&mut self, reg_idx: u8, value: u8) {
        self.reg_vx[reg_idx as usize] = value;
    }

    fn _get_vx(&self, reg_idx: u8) -> u8 {
        self.reg_vx[reg_idx as usize]
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "----------------------\n")?;
        write!(f, "PC:{:x} I:{:x} SP:{:}\n", self.prog_cnt, self.reg_idx, self.stack_ptr)?;
        write!(f, "{:<8}| {:<8} {:<8}\n", "idx", "reg_vx", "stack")?;
        for i in 0..16 {
            write!(f, "{:<8x}| {:<8x} {:<8x}", i, self.reg_vx[i], self.stack[i])?;
            if self.stack_ptr as usize == i {
                write!(f, "<-")?;
            }
            write!(f, "\n")?;
        }
        write!(f, "----------------------\n")
    }
}
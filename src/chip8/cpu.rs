// TODO(#4): super chip8 instruction
// an extension of standard chip8 instruction
use std::fmt;

use super::ram::{Ram, RAM_START_OFFSET};
use super::display::Display;

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
            prog_cnt:   RAM_START_OFFSET,
            stack:      [0; 16],
            stack_ptr:  0,
        }
    }

    pub fn read_instr(&self, ram: &Ram) -> u16{
        let hi = ram.read(self.prog_cnt as usize) as u16;
        let lo = ram.read(1 + self.prog_cnt as usize) as u16;

        lo | (hi << 8)
    }

    pub fn exec_instr(&mut self, ram: &mut Ram, display: &mut Display, op_code: u16) {
        let nnn = op_code & 0x0FFF as u16;
        let n = (op_code & 0x000F) as u8;
        let kk = (op_code & 0x00FF) as u8;
        let x = ((op_code & 0x0F00) >> 8) as u8;
        let y = ((op_code & 0x00F0) >> 4) as u8;

        let op_tup = (
            ((op_code & 0xF000) >> 12),
            ((op_code & 0x0F00) >> 8),
            ((op_code & 0x00F0) >> 4),
            ((op_code & 0x000F) >> 0), 
        );

        match op_tup {
            (0x0, 0x0, 0xE, 0x0) => self.exec_00e0(display),
            (0x0, 0x0, 0xE, 0xE) => self.exec_00ee(),
            (0x1,   _,   _,   _) => self.exec_1nnn(nnn),
            (0x2,   _,   _,   _) => self.exec_2nnn(nnn),
            (0x3,   _,   _,   _) => self.exec_3xkk(x, kk),
            (0x4,   _,   _,   _) => self.exec_4xkk(x, kk),
            (0x6,   _,   _,   _) => self.exec_6xkk(x, kk),
            (0x7,   _,   _,   _) => self.exec_7xkk(x, kk),
            (0x8,   _,   _, 0x0) => self.exec_8xy0(x, y),
            (0x8,   _,   _, 0x2) => self.exec_8xy2(x, y),
            (0x8,   _,   _, 0x7) => self.exec_8xy7(x, y),
            (0x8,   _,   _, 0xE) => self.exec_8x0e(x),
            (0x9,   _,   _, 0x0) => self.exec_9xy0(x, y),
            (0xA,   _,   _,   _) => self.exec_annn(nnn),
            (0xD,   _,   _,   _) => self.exec_dxyn(display, ram, x, y, n),
            (0xF,   _, 0x0, 0x7) => self.exec_fx07(x),
            (0xF,   _, 0x1, 0x5) => self.exec_fx15(x),
            (0xF,   _, 0x2, 0x9) => self.exec_fx29(x),
            (0xF,   _, 0x3, 0x3) => self.exec_fx33(ram, x),
            (0xF,   _, 0x6, 0x5) => self.exec_fx65(ram, x),
            _ => { panic!("OPCode unknown found: 0x{:04x?} tup: {:?}", op_code, op_tup); }
        }
    }
}

impl Cpu {
    fn stack_push(&mut self, value: u16) {
        self.stack[self.stack_ptr as usize] = value;
        self.stack_ptr += 1;
    }

    fn stack_pop(&mut self) -> u16 {
        self.stack_ptr -= 1;
        self.stack[self.stack_ptr as usize]
    }

    fn exec_00e0(&mut self, display: &mut Display) {
        println!("CLR");
        display.clear();
        self.prog_cnt += 2;
    }

    fn exec_2nnn(&mut self, addr: u16) {
        println!("Call subroutine at 0x{:04x?}", addr);
        self.stack_push(self.prog_cnt);
        self.prog_cnt = addr;
    }

    fn exec_00ee(&mut self) {
        println!("RET");
        self.prog_cnt = self.stack_pop();
        self.prog_cnt += 2;
    }

    fn exec_1nnn(&mut self, nnn: u16) {
        println!("JMP to nnn(0x{:04x?})", nnn);
        self.prog_cnt = nnn;
    }

    fn exec_3xkk(&mut self, x: u8, kk: u8) {
        println!("Skip next instruction if V{:x?} = kk({:x?}).", x, kk);
        self.prog_cnt += 
            if self.reg_vx[x as usize] == kk {
                4
            } else {
                2
            }
    }

    fn exec_4xkk(&mut self, x: u8, kk: u8) {
        println!("Skip next instruction if V{:x?} != kk({:x?})", x, kk);
        self.prog_cnt += 
            if self.reg_vx[x as usize] != kk {
                4
            } else {
                2
            }
      }

      fn exec_6xkk  (&mut self, x: u8, kk: u8) {
        println!("Set V{:x?} = {:02x?}", x, kk);
        self.reg_vx[x as usize] = kk;
        self.prog_cnt += 2;
    }

    fn exec_7xkk(&mut self, x: u8, kk: u8) {
        println!("Set V{:x?} = V{:x?} + {:02x?}", x, x, kk);
        self.reg_vx[x as usize] = self.reg_vx[x as usize].wrapping_add(kk);
        self.prog_cnt += 2;
    }

    fn exec_8xy0(&mut self, x:u8, y: u8) {
        println!("Set V{:x?} = V{:x?}", x, y);
        self.reg_vx[x as usize] = self.reg_vx[y as usize];
          self.prog_cnt +=   2;
      }

    fn exec_8xy2(&mut self, x:u8, y: u8) {
        println!("Set Vx = Vx AND Vy.");
        self.reg_vx[x as usize] &= self.reg_vx[y as usize];
        self.prog_cnt += 2;
    }

    fn exec_8xy7(&mut self, x: u8, y: u8) {
        println!("Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.");
        self.reg_vx[x as usize] = self.reg_vx[y as usize].wrapping_sub(self.reg_vx[x as usize]);
        if self.reg_vx[x as usize] > self.reg_vx[y as usize] {
            self.reg_vx[0xF] = 1
        }
        else {
            self.reg_vx[0xF] = 0
        }
        self.prog_cnt += 2;
    }

    fn exec_8x0e(&mut self, x: u8) {
        println!("Set V{:x?} = V{:x?} SHL 1.", x, x);
        self.reg_vx[0xF] = self.reg_vx[x as usize] & 0b1000_0000;
        self.reg_vx[x as usize] <<= 1;
        self.prog_cnt += 2;
    }

    fn exec_9xy0(&mut self, x: u8, y: u8) {
        println!("Skip next instruction if V{:x?} != V{:x?}", x, y);
        self.prog_cnt += 
            if self.reg_vx[x as usize] != self.reg_vx[y as usize] {
                4
            } else {
                2
            }
    }

    fn exec_annn(&mut self, nnn: u16) {
        println!("Set I = {:04x?}", nnn);
        self.reg_idx = nnn;
        self.prog_cnt += 2;
    }

    fn exec_dxyn(&mut self, display: &mut Display, ram: &mut Ram,x: u8,  y: u8, n: u8) {
        let pos_x = self.reg_vx[x as usize] as usize;
        let pos_y = self.reg_vx[y as usize] as usize;
        let sprite_start_idx = self.reg_idx as usize;
        let sprite_end_idx = self.reg_idx as usize + n as usize;
        println!("Display {}-byte sprite starting at memory location I({}) at (V{}, V{}), set VF = x.", n, self.reg_idx ,x, y);
        
        if display.fill_screen(
            &ram.memory[sprite_start_idx..sprite_end_idx], pos_x, pos_y) == true {
            self.reg_vx[0xF] = 0x01;
        }
        else {
            self.reg_vx[0xF] = 0x00;
        }
            
        self.prog_cnt += 2;
    }

    fn exec_fx07(&mut self, x: u8) {
        println!("not impl: Set V{:x?} = delay timer value.", x);
        self.prog_cnt += 2;
    }

    fn exec_fx15(&mut self, x: u8) {
        println!("not impl: Set delay timer = V{:x?}.", x);
        self.prog_cnt += 2;
    }

    fn exec_fx29(&mut self, x :u8) {
        println!("Set I = location of sprite for digit V{:x?}.", x);
        self.reg_idx = (self.reg_vx[x as usize] as u16) * 5;
        self.prog_cnt += 2;
    }

    fn exec_fx33(&mut self, ram: &mut Ram, x: u8) {
        println!("Store BCD representation of V{:x?} in memory locations I, I+1, and I+2.", x);
        ram.write(self.reg_idx as usize, self.reg_vx[x as usize] / 100);
        ram.write((self.reg_idx as usize) + 1, (self.reg_vx[x as usize] % 100) /10);
        ram.write((self.reg_idx as usize) + 2, self.reg_vx[x as usize] % 10);
        self.prog_cnt += 2;
    }

    fn exec_fx65(&mut self, ram: &mut Ram, x: u8) {
        println!("Read registers V0 through Vx from memory starting at location I.");
        for i in 0..x {
            //ram.write((self.reg_idx + i as u16) as usize, self.reg_vx[i]);
            self.reg_vx[i as usize] = ram.read((self.reg_idx + i as u16) as usize);
        }
        self.prog_cnt += 2;
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

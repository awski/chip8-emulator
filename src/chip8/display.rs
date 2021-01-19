use std::fmt;
use super::ram;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const SCREEN_FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Display {
    pub screen: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Display {
    pub fn new() -> Display{
        Display {
            screen: [[0u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }
    pub fn load_fonts(ram: &mut ram::Ram) {
        for i in 0..SCREEN_FONT.len() {
            ram.write(i, SCREEN_FONT[i]);
        }
    }
    pub fn fill_screen(&mut self, sprite: &[u8], pos_x: usize, pos_y: usize) -> bool{
        let h = sprite.len();
        let mut col = false;
        for j in 0..h {
            for i in 0..8 {
                let local_x = (pos_x + i) % SCREEN_WIDTH;
                let local_y = (pos_y + j) % SCREEN_HEIGHT;
                if (sprite[j] & (0b1000_0000 >> i)) != 0b0000_0000 {
                    if self.screen[local_y][local_x] == 0b0000_0001 {
                        col = true;
                    }
                    self.screen[local_y][local_x] ^= 0b0000_0001;
                }
            }
        }
        col
    }

    pub fn clear(&mut self) {
        self.screen = [[0u8; SCREEN_WIDTH]; SCREEN_HEIGHT];
    }

    pub fn get_pixel(&self, pos_x: usize, pos_y: usize) -> u8 {
        assert!(pos_x < SCREEN_WIDTH && pos_y < SCREEN_HEIGHT);
        self.screen[pos_y][pos_x]
    }
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                if self.screen[y][x] != 0 {
                    write!(f, "#")?;
                }
                else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        
        write!(f,"")
    }
}
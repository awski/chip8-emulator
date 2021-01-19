use super::cpu::Cpu;
use super::ram::Ram;
use super::display::{Display, SCREEN_HEIGHT, SCREEN_WIDTH};

#[test]
fn test_display_load_fonts() {
    let mut ram = Ram::new();

    Display::load_fonts(&mut ram);

    let mut val = ram.read(0);
    assert!(val == 0xF0);

    val = ram.read(4);
    assert!(val == 0xF0);

    val = ram.read(74);
    assert!(val == 0xF0);

    val = ram.read(79);
    assert!(val == 0x80);
}

#[test]
fn test_display_clear() {
    let mut disp = Display::new();

    disp.clear();

    let mut val = disp.get_pixel(0, 0);
    assert!(val == 0u8);

    val = disp.get_pixel(SCREEN_WIDTH - 1, 0);
    assert!(val == 0u8);

    val = disp.get_pixel(0, SCREEN_HEIGHT - 1);
    assert!(val == 0u8);
}

#[test]
#[should_panic]
fn test_display_illegal_get_pixel() {
    let disp = Display::new();
    disp.get_pixel(SCREEN_WIDTH, SCREEN_HEIGHT);
}
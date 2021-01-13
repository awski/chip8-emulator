use super::cpu;
use super ::ram;

#[test]
fn test_01() {
    let c = cpu::Cpu::new();
}

fn test_02() {
    let r = ram::Ram::new();
    r.read(0);
}
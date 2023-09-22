use core::arch::asm;

/// 4
pub fn iocsr_write_w(reg: usize, value: u32) {
    unsafe {
        asm!("iocsrwr.w {},{}", in(reg) value, in(reg) reg);
    }
}
// 8
pub fn iocsr_write_d(reg: usize, value: u64) {
    unsafe {
        asm!("iocsrwr.d {},{}", in(reg) value, in(reg) reg);
    }
}
// 2
pub fn iocsr_write_h(reg: usize, value: u16) {
    unsafe {
        asm!("iocsrwr.h {},{}", in(reg) value, in(reg) reg);
    }
}
// 1
pub fn iocsr_write_b(reg: usize, value: u8) {
    unsafe {
        asm!("iocsrwr.b {},{}", in(reg) value, in(reg) reg);
    }
}

// 1
pub fn iocsr_read_b(reg: usize) -> u8 {
    let val: u8;
    unsafe {
        asm!("iocsrrd.b {},{}",out(reg) val, in(reg) reg);
    }
    val
}

// 2
pub fn iocsr_read_h(reg: usize) -> u16 {
    let val: u16;
    unsafe {
        asm!("iocsrrd.h {},{}",out(reg) val, in(reg) reg);
    }
    val
}

// 4
pub fn iocsr_read_w(reg: usize) -> u32 {
    let val: u32;
    unsafe {
        asm!("iocsrrd.w {},{}",out(reg) val, in(reg) reg);
    }
    val
}

// 8
pub fn iocsr_read_d(reg: usize) -> u64 {
    let val: u64;
    unsafe {
        asm!("iocsrrd.d {},{}",out(reg) val, in(reg) reg);
    }
    val
}

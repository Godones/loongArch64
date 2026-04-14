#![no_std]
#![allow(unused)]
#![allow(non_snake_case)]
pub mod asm;
pub mod consts;
pub mod cpu;
pub mod iocsr;
pub mod ipi;
pub mod register;
pub mod time;
const VALEN: usize = 48;
const PALEN: usize = 48;

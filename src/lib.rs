#![no_std]
#![feature(asm_const)]
#![allow(unused)]
pub mod asm;
pub mod cpu;
pub mod iocsr;
pub mod ipi;
pub mod register;
pub mod time;
pub mod consts;
const VALEN: usize = 48;
const PALEN: usize = 48;

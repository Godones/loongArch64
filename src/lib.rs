#![no_std]
#![feature(asm_const)]
#![allow(unused)]
pub mod asm;
mod consts;
pub mod cpu;
pub mod iocsr;
pub mod ipi;
pub mod register;
pub mod time;

const VALEN: usize = 48;
const PALEN: usize = 48;

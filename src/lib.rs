#![no_std]
#![feature(asm_const)]
#![allow(unused)]
mod consts;
pub mod cpu;
pub mod iocsr;
pub mod ipi;
pub mod register;
pub mod time;
const VALEN: usize = 48;
const PALEN: usize = 48;

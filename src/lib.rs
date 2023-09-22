#![no_std]
#![feature(asm_const)]
#![allow(unused)]
pub mod register;
pub mod cpu;
pub mod time;
pub mod ipi;
mod consts;
pub mod iocsr;
const VALEN:usize = 48;
const PALEN:usize = 48;

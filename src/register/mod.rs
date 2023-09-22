#[macro_use]
mod macros;
mod base;
mod mmu;
mod ras;
mod timer;

pub use base::*;
use core::fmt::{Display, Formatter};
pub use mmu::*;
pub use timer::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MemoryAccessType {
    StronglyOrderedUnCached = 0,
    CoherentCached = 1,
    WeaklyOrderedUnCached = 2,
}
impl Display for MemoryAccessType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            MemoryAccessType::StronglyOrderedUnCached => write!(f, "StronglyOrderedUnCached"),
            MemoryAccessType::CoherentCached => write!(f, "CoherentCached"),
            MemoryAccessType::WeaklyOrderedUnCached => write!(f, "WeaklyOrderedUnCached"),
        }
    }
}
impl From<usize> for MemoryAccessType {
    fn from(value: usize) -> Self {
        match value {
            0 => MemoryAccessType::StronglyOrderedUnCached,
            1 => MemoryAccessType::CoherentCached,
            2 => MemoryAccessType::WeaklyOrderedUnCached,
            _ => panic!("Invalid MemoryAccessType value: {}", value),
        }
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum CpuMode {
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3,
}

impl From<usize> for CpuMode {
    fn from(value: usize) -> Self {
        match value {
            0 => CpuMode::Ring0,
            1 => CpuMode::Ring1,
            2 => CpuMode::Ring2,
            3 => CpuMode::Ring3,
            _ => panic!("Invalid CpuMode value: {}", value),
        }
    }
}

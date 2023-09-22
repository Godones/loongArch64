use bit_field::BitField;
use core::fmt::Debug;
impl_define_csr!(Rvacfg, "Reduced Virtual Address Configuration\n\
                          This register is used to control the length of the address being reduced in the virtual address reduction mode.");
impl_read_csr!(0x1f, Rvacfg);
impl Rvacfg {
    /// The number of the high order bits of the address to be reduced in the virtual address reduction mode.
    /// It can be configured to a value between 0 and 8.
    /// Specially, 0 means that the virtual address reduction mode is disabled.
    /// The processor behavior with `rbits` over 8 is undefined.
    fn rbits(&self) -> usize {
        self.bits
    }
}

/// The number of the high order bits of the address to be reduced in the virtual address reduction mode.
/// It can be configured to a value between 0 and 8.
/// Specially, 0 means that the virtual address reduction mode is disabled.
/// # Warning!
/// The processor behavior with `rbits` over 8 is UNDEFINED.

pub fn set_rbits(val: usize) {
    debug_assert!(val <= 8);
    set_csr_loong_bits!(0x1f, 0.., val);
}

impl Debug for Rvacfg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RVACfg")
            .field("rbits", &self.rbits())
            .finish()
    }
}

use core::fmt::Debug;
impl_read_csr!(0x89, TlbRBadv);
impl_define_csr!(
    TlbRBadv,
    "TLB Refill Exception Bad Virtual Address (TLBRBADV)

This register is used to record the bad virtual address that triggered the TLB refill exception.
"
);
impl Debug for TlbRBadv {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TLBRBadV")
            .field("bits", &format_args!("{:X}", self.bits))
            .finish()
    }
}

impl TlbRBadv {
    /// When the TLB refill exception is triggered, the hardware records the bad virtual address here.
    /// For LA64, in this case, if the privilege level that triggered the exception is in 32-bit address mode,
    /// then the high 32 bits of the recorded virtual address will be set to 0.

    pub fn vaddr(&self) -> usize {
        self.bits
    }
}

/// When the TLB refill exception is triggered, the hardware records the bad virtual address here.
/// For LA64, in this case, if the privilege level that triggered the exception is in 32-bit address mode,
/// then the high 32 bits of the recorded virtual address will be set to 0.

pub fn set_vaddr(value: usize) {
    write_csr_loong!(0x89, value);
}

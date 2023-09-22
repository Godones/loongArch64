impl_define_csr!(Pgd, " Page Global Directory Base Address (PGD)
This register is a read-only register.
Store global directory base address information corresponding to the bad virtual address in the current context.
");

impl_read_csr!(0x1b, Pgd);

impl Pgd {
    /// If the highest bit of the bad virtual address(`BadV`) in the current context is 0:
    /// * the return value of reading is equal to the Base field of `CSR.PGDL`;
    /// OTHERWISE,
    /// * the read return value is equal to the Base field of `CSR.PGDH`.
    ///
    /// When `CSR.TLBRERA.IsTLBR`=0,
    /// * the bad virtual address information in the current context is located in `CSR.BADV`;
    /// OTHERWISE,
    /// * the bad virtual address information is located in `CSR.TLBRBADV`.
    pub fn base(&self) -> usize {
        self.bits
    }
}
impl core::fmt::Debug for Pgd {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PGD")
            .field("bits", &format_args!("{:X}", self.bits))
            .finish()
    }
}

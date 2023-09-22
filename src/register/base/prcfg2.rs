impl_define_csr!(Prcfg2, "Privileged Resource Configuration 1");
impl_read_csr!(0x22, Prcfg2);

impl Prcfg2 {
    /// Return a bit vector of page sizes supported by the TLB.
    ///
    /// If the bit is 1, the 2^(i) page size is supported.
    pub fn psval(&self) -> usize {
        self.bits
    }
}

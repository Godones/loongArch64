use bit_field::BitField;
impl_define_csr!(Eentry, "Exception Entry Base Address CSR\n\
                          This register is used to configure the entry base address for general exceptions and interrupts.");

impl_read_csr!(0xc, Eentry);

impl Eentry {
    /// Returns the entry base address
    pub fn eentry(&self) -> usize {
        self.bits
    }
}

pub fn set_eentry(eentry: usize) {
    debug_assert_eq!(eentry & 0xfff, 0);
    set_csr_loong_bits!(0xc, 0.., eentry);
}

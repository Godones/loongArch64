use bit_field::BitField;
impl_read_csr!(0x1e, StlbPs);
impl_define_csr!(
    StlbPs,
    "STLB Page Size (STLBPS)

This register is used to configure the size of the page in the STLB.
"
);

impl StlbPs {
    /// Get the `log(RealPageSize)/log(2)`
    /// For example, if the page size is 16KB, then `PS`=`0xE`.
    pub fn ps(&self) -> usize {
        self.bits.get_bits(0..=5)
    }
}
/// Set the `log(RealPageSize)/log(2)`
/// For example, if the page size is 16KB, then `PS`=`0xE`.

pub fn set_ps(page_size: usize) {
    set_csr_loong_bits!(0x1e, 0..=5, page_size);
}

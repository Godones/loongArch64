use bit_field::BitField;
impl_define_csr!(StlbPs);
impl_read_csr!(0x1e,StlbPs);
impl_write_csr!(0x1e,StlbPs);


impl StlbPs {
    pub fn ps(&self) -> usize {
        self.bits.get_bits(0..=5)
    }
    pub fn set_ps(&mut self, page_size: usize) -> &mut Self {
        self.bits.set_bits(0..=5, page_size);
        self
    }
}

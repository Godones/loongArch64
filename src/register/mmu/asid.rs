use bit_field::BitField;
impl_define_csr!(Asid);
impl_read_csr!(0x18,Asid);
impl_write_csr!(0x18,Asid);


impl Asid {
    pub fn asid(&self) -> usize {
        self.bits.get_bits(0..10)
    }
    pub fn set_asid(&mut self, asid: usize) -> &mut Self {
        self.bits.set_bits(0..10, asid);
        self
    }

    pub fn asid_width(&self) -> usize {
        self.bits.get_bits(16..=23)
    }
    pub fn set_asid_width(&mut self, asid_width: usize) -> &mut Self {
        self.bits.set_bits(16..=23, asid_width);
        self
    }
}
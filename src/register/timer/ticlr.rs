use bit_field::BitField;
impl_define_csr!(Ticlr);
impl_read_csr!(0x44,Ticlr);


impl Ticlr {
    pub fn clear_timer(&mut self) -> &mut Self {
        self.bits.set_bit(0, true);
        self
    }
}

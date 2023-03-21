impl_define_csr!(Badi);
impl_write_csr!(0x8,Badi);
impl_read_csr!(0x8,Badi);


impl Badi {
    pub fn inst(&self) -> usize {
        self.bits
    }
}
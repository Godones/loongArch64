impl_define_csr!(TlbRSave);
impl_write_csr!(0x8b,TlbRSave);
impl_read_csr!(0x8b,TlbRSave);

impl TlbRSave {
    pub fn data(&self)->usize{
        self.bits
    }
    pub fn set_data(&mut self, value: usize) -> &mut Self {
        self.bits = value;
        self
    }
}
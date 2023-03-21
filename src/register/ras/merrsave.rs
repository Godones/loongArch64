impl_define_csr!(MerrSave);
impl_write_csr!(0x95,MerrSave);
impl_read_csr!(0x95,MerrSave);

impl MerrSave{
    pub fn data(&self)->usize{
        self.bits
    }
    pub fn set_data(&mut self, data: usize)->&mut Self{
        self.bits = data;
        self
    }
}

impl_define_csr!(Cntc);
impl_read_csr!(0x43,Cntc);
impl_write_csr!(0x43,Cntc);

impl Cntc{
    pub fn compensation(&self)->usize{
        self.bits
    }
    pub fn set_compensation(&mut self,compensation:usize)->&mut Self{
        self.bits=compensation;
        self
    }
}
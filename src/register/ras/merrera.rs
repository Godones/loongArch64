impl_define_csr!(MerrEra);
impl_read_csr!(0x94,MerrEra);
impl_write_csr!(0x94,MerrEra);

impl MerrEra{
    pub fn pc(&self)->usize{
        self.bits
    }
    pub fn set_pc(&mut self, pc: usize)->&mut Self{
        self.bits = pc;
        self
    }

}
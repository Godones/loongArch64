
impl_define_csr!(Tval);
impl_read_csr!(0x42,Tval);

impl Tval{
    pub fn time_val(&self)->usize{
        self.bits
    }
}
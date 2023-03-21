
impl_define_csr!(Tid);
impl_read_csr!(0x40,Tid);
impl_write_csr!(0x40,Tid);

impl Tid{
    pub fn tid(&self)->usize{
        self.bits
    }
}
impl_define_csr!(Badv);
impl_write_csr!(0x7,Badv);
impl_read_csr!(0x7,Badv);

impl Badv {
    pub fn vaddr(&self) -> usize {
        self.bits
    }
}

impl_define_csr!(Eentry);
impl_write_csr!(0xc,Eentry);
impl_read_csr!(0xc,Eentry);

impl Eentry {
    pub fn eentry(&self) -> usize {
        // 12位以后,以页对齐
        self.bits
    }
    pub fn set_eentry(&mut self, eentry: usize) -> &mut Self {
        assert_eq!(eentry & 0xfff, 0);
        self.bits = eentry;
        self
    }
}
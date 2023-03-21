impl_define_csr!(Pgd);
impl_read_csr!(0x1b,Pgd);

impl Pgd {
    pub fn base(&self) -> usize {
        self.bits
    }
}

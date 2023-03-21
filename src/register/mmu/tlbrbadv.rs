impl_define_csr!(TlbRBadv);
impl_read_csr!(0x89,TlbRBadv);
impl_write_csr!(0x89,TlbRBadv);

impl TlbRBadv {
    pub fn vaddr(&self) -> usize {
        self.bits
    }
    pub fn set_vaddr(&mut self, value: usize) -> &mut Self {
        self.bits = value;
        self
    }
}

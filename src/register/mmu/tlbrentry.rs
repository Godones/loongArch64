impl_define_csr!(TlbREntry);
impl_read_csr!(0x88,TlbREntry);
impl_write_csr!(0x88,TlbREntry);

impl TlbREntry {
    pub fn addr(&self) -> usize {
        self.bits
    }
    pub fn set_addr(&mut self, val: usize) -> &mut Self {
        // 对齐到4kb
        assert_eq!(val & 0xFFF, 0);
        self.bits = val;
        self
    }
}

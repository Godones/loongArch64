impl_define_csr!(Pgdl);
impl_read_csr!(0x19,Pgdl);
impl_write_csr!(0x19,Pgdl);

impl Pgdl {
    pub fn base(&self) -> usize {
        self.bits
    }
    pub fn set_base(&mut self, val: usize) -> &mut Self {
        // 确保地址是 4KB 边界地址对齐的
        assert_eq!(val & 0xFFF, 0);
        self.bits = val;
        self
    }
}
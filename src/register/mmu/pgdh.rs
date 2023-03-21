impl_define_csr!(Pgdh);
impl_read_csr!(0x1a,Pgdh);
impl_write_csr!(0x1a,Pgdh);

impl Pgdh {
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

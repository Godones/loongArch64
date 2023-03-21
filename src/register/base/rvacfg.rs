
impl_define_csr!(Rvacfg);
impl_write_csr!(0x1f,Rvacfg);
impl_read_csr!(0x1f,Rvacfg);

impl Rvacfg {
    fn rbits(&self) -> usize {
        self.bits
    }
    // 虚地址缩减模式下，被缩减的高位地址的位数。可以配置为 0~8 之间的值。
    // 0 是一个特殊的配置值，意味着不启用虚地址缩减模式。
    // 如果配置的值大于 8，则处理器行为不确定
    fn set_rbits(&mut self, val: usize) -> &mut Self {
        self.bits = val;
        self
    }
}

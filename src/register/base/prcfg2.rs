
impl_define_csr!(Prcfg2);
impl_read_csr!(0x22,Prcfg2);


// 指示 TLB 能够支持的页大小（Page Size）。当第 i 位为 1，表明支持 2
// i字节大小的页
impl Prcfg2 {
    pub fn psval(&self) -> usize {
        self.bits
    }
}

// 无论 CSR.TLBRERA.IsTLBR 等于何值，执行 TLBRD 指令都只更新 TLBEHI 寄存器

use bit_field::BitField;
impl_define_csr!(TlbREhi);
impl_read_csr!(0x8e,TlbREhi);
impl_write_csr!(0x8e,TlbREhi);



impl TlbREhi {
    // TLB 重填例外专用的页大小值。即在 CSR.TLBRERA.IsTLBR=1 时，执行 TLBWR 和 TLBFILL
    // 指令，写入的 TLB 表项的 PS 域的值来自于此。
    pub fn ps(&self) -> usize {
        self.bits.get_bits(0..=5)
    }
    pub fn set_ps(&mut self, page_size: usize) -> &mut Self {
        self.bits.set_bits(0..=5, page_size);
        self
    }
    pub fn vppn(&self, valen: usize) -> usize {
        self.bits.get_bits(13..valen)
    }
    pub fn set_vppn(&mut self, valen: usize, vppn: usize) -> &mut Self {
        self.bits.set_bits(13..valen, vppn);
        self
    }
}

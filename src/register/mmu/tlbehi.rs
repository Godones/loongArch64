use bit_field::BitField;
use crate::VALEN;
impl_define_csr!(TlbEhi);
impl_read_csr!(0x11,TlbEhi);
impl_write_csr!(0x11,TlbEhi);

impl TlbEhi {
    // 执行 TLBRD 指令时，所读取 TLB 表项的 VPPN 域的值记录到这里。
    // 在 CSR.TLBRERA.IsTLBR=0 时，执行 TLBSRCH 指令时查询 TLB 所用 VPPN 值，以及执行
    // TLBWR 和 TLBFILL 指令时写入 TLB 表项的 VPPN 域的值来自于此。
    // 当触发 load 操作页无效例外、store 操作页无效例外、取指操作页无效例外、页修
    // 改例外、页不可读例外、页不可执行例外和页特权等级不合规例外时，触发例外的地址的[VALEN-1:13]位被记录到这里。
    pub fn vppn(&self) -> usize {
        self.bits.get_bits(13..VALEN)
    }
    pub fn set_vppn(&mut self, valen: usize, vppn: usize) -> &mut Self {
        self.bits.set_bits(13..valen, vppn);
        self
    }
}

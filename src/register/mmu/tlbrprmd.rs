// 当触发 TLB 重填例外时，硬件会将此时处理器核的特权等级、客户机模式、全局中断使能和监视点使
// 能位保存至该寄存器中，用于例外返回时恢复处理器核的现场


use bit_field::BitField;
use crate::register::CpuMode;
impl_define_csr!(TlbRPrmd);
impl_write_csr!(0x8f,TlbRPrmd);
impl_read_csr!(0x8f,TlbRPrmd);

impl TlbRPrmd {
    pub fn pplv(&self) -> usize {
        self.bits.get_bits(0..2)
    }
    pub fn set_pplv(&mut self, pplv: CpuMode) -> &mut Self {
        //设置特权级
        // 用于在进入用户程序时设置特权级
        self.bits.set_bits(0..2, pplv as usize);
        self
    }
    // 记录例外发生前的crmd.ie
    pub fn pie(&self) -> bool {
        self.bits.get_bit(2)
    }
    // 设置中断使能
    // 用于在进入用户程序时设置中断使能
    pub fn set_pie(&mut self, pie: bool) -> &mut Self {
        self.bits.set_bit(2, pie);
        self
    }

    pub fn pwe(&self) -> bool {
        self.bits.get_bit(4)
    }
    pub fn set_pwe(&mut self, pwe: bool) -> &mut Self {
        self.bits.set_bit(4, pwe);
        self
    }
}

// 该寄存器保存 TLB 重填例外处理完毕之后的返回地址。除此之外，该寄存器还包含用于标识当前例外
// 是 TLB 重填例外的标志位

use bit_field::BitField;
impl_define_csr!(TlbREra);
impl_read_csr!(0x8a,TlbREra);
impl_write_csr!(0x8a,TlbREra);


impl TlbREra {
    // 记录触发 TLB 重填例外的指令的 PC 的[GRLEN-1:2]位。当执行 ERTN 指令从 TLB 重填
    // 例外处理程序返回时（此时本寄存器 IsTLBR=1 且 CSR.ERRCTL.IsMERR=0），硬件自动
    // 将存放在此处的值最低位补上两比特 0 后作为最终的返回地址
    pub fn pc(&self) -> usize {
        // 返回pc
        self.bits.get_bits(2..)
    }
    pub fn set_pc(&mut self, pc: usize) -> &mut Self {
        // 设置pc
        self.bits.set_bits(2.., pc);
        self
    }
    pub fn is_tlbr(&self) -> bool {
        // 返回是否是 TLB 重填例外
        self.bits.get_bit(0)
    }
    pub fn set_is_tlbr(&mut self, is_tlbr: bool) -> &mut Self {
        self.bits.set_bit(0, is_tlbr);
        self
    }
}

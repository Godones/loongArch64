use bit_field::BitField;
impl_define_csr!(TlbIdx);
impl_read_csr!(0x10,TlbIdx);
impl_write_csr!(0x10,TlbIdx);


impl TlbIdx {
    // 执行 TLBRD 和 TLBWR 指令时，访问 TLB 表项的索引值来自于此。
    // 执行 TLBSRCH 指令时，如果命中，则命中项的索引值记录到这里
    pub fn index(&self) -> usize {
        self.bits.get_bits(0..16)
    }
    pub fn set_index(&mut self, index: usize) -> &mut Self {
        self.bits.set_bits(0..16, index);
        self
    }
    // 执行 TLBRD 指令时，所读取 TLB 表项的 PS 域的值记录到这里。
    // 在 CSR.TLBRERA.IsTLBR=0 时，执行 TLBWR 和 TLBFILL 指令，写入的 TLB 表项的 PS
    // 域的值来自于此。
    pub fn ps(&self) -> usize {
        self.bits.get_bits(24..=29)
    }
    pub fn set_ps(&mut self, ps: usize) -> &mut Self {
        self.bits.set_bits(24..=29, ps);
        self
    }
    // 该位为 1 表示该 TLB 表项为空（无效 TLB 表项），为 0 表示该 TLB 表项非空（有效 TLB
    // 表项）。
    // 执行 TLBSRCH 时，如果有命中项该位记为 0，否则该位记为 1。
    // 执行 TLBRD 时，所读取 TLB 表项的 E 位信息取反后记录到这里。
    // 执行 TLBWR 或 TLBFILL 指令时，若 CSR.TLBRERA.IsTLBR=0，将该位的值取反后写入
    // 到被写 TLB 项的 E 位；若此时 CSR.TLBRERA.IsTLBR=1，那么被写入的 TLB 项的 E 位
    // 总是置为 1，与该位的值无关
    pub fn ne(&self) -> bool {
        self.bits.get_bit(31)
    }
    pub fn set_ne(&mut self, ne: bool) -> &mut Self {
        self.bits.set_bit(31, ne);
        self
    }
}

use bit_field::BitField;

impl_define_csr!(Prcfg3, "Privileged Resource Configuration 1");

impl_read_csr!(0x23, Prcfg3);

impl Prcfg3 {
    /// 指示 TLB 组织方式：
    /// # Return Values:
    /// * 0：No TLB
    /// * 1：一个全相联的多重页大小 TLB（MTLB）
    /// * 2：一个全相联的多重页大小 TLB（MTLB）+一个组相联的单个页大小 TLB（STLB）；
    /// * Others: Reserved.
    pub fn tlb_type(&self) -> usize {
        self.bits.get_bits(0..=3)
    }
    /// 当 TLBType=1 或 2 时，该域的值是全相联多重页大小 TLB 的项数减 1
    pub fn mtlb_entries(&self) -> usize {
        self.bits.get_bits(4..=11)
    }

    /// STLBWays
    pub fn stlb_ways(&self) -> usize {
        self.bits.get_bits(12..=19) + 1
    }

    /// 当 TLBType=2 时，该域的值是组相联单个页大小 TLB 的每一路项数的幂指数，即每一
    /// 路有 2 ^ STLBSets项。
    pub fn sltb_sets(&self) -> usize {
        self.bits.get_bits(20..=25)
    }
}

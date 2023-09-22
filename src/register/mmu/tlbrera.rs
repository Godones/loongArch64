use bit_field::BitField;
use core::fmt::Debug;
impl_define_csr!(TlbREra,"TLB Refill Exception Return Address,\n\
                          This register is used to record the PC of the instruction that triggered the TLB refill exception.\n\
                          In addition, this register contains flag bits to identify the current exception as a TLB refill exception.");

impl_read_csr!(0x8a, TlbREra);

impl TlbREra {
    /// Record the [GRLEN-1:2] bits of the PC of the instruction that triggered the TLB refill exception.
    /// When the execution of ERTN instruction returns from the TLB refill exception handler.
    /// (at this time, this register IsTLBR=1 and CSR.MERRCTL.IsMERR=0)
    pub fn pc(&self) -> usize {
        // 返回pc
        self.bits.get_bits(2..)
    }
    /// 1 indicates that it is currently in the context of TLB refill exception processing.
    /// The hardware sets this bit to 1 when a TLB refill exception is triggered.
    /// When this bit is 1, execution of the ERTN instruction will clear it to 0 only if CSR.MERRCTL.IsMERR=0, otherwise it remains unchanged.
    /// Because the architecture defines a separate set of CSRs for TLB refill exceptions, when this bit is 1.
    /// * When ERTN returns, the information used to recover CSR.CRMD will come from CSR.TLBRPRMD;
    /// * ERTN return address will come from CSR.TLBRERA;
    /// * The table entries to be written by TLBWR and TLBFILL instructions will come from CSR.TLBREHI, CSR.TLBELO0 and CSR.TLBELO1;
    /// * TLBSRCH instruction queries information from CSR.TLBREHI;
    /// * The bad virtual address required for LDDIR and LDPTE instruction execution will come from CSR.TLBRBADV.
    pub fn is_tlbr(&self) -> bool {
        self.bits.get_bit(0)
    }
}
impl Debug for TlbREra {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TLBRERA")
            .field("pc", &format_args!("{:X}", self.pc()))
            .field("is_tlbr", &self.is_tlbr())
            .finish()
    }
}

/// Set the TLB refill exception flag bit.
/// See also: [TlbREra::is_tlbr]
pub fn set_is_tlbr(is_tlbr: bool) {
    set_csr_loong_bit!(0x8a, 0, is_tlbr);
}

/// Set the PC of the instruction that triggered the TLB refill exception.
///
/// See also: [TlbREra::pc]
pub fn set_pc(pc: usize) {
    set_csr_loong_bits!(0x8a, 2.., pc);
}

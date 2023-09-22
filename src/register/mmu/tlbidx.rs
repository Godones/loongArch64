use bit_field::BitField;

impl_read_csr!(0x10, TlbIdx);
impl_define_csr!(TlbIdx, "TLB Index (TLBIDX)

This register contains information such as the index associated with the TLB-related instruction.
The length of the Index field in the table depends on implementation,
although LoongArch allows for an Index length of no more than 16 bits.

This register also contains the information related to the PS and P fields in the TLB table entry when executing TLB-related instructions.
");

impl TlbIdx {
    /// When executing the TLBRD and TLBWR instructions, the index of the access TLB table entry comes from here.
    ///
    /// When executing the TLBSRCH instruction, if it hits, the index of the hit entry is recorded here.
    ///
    /// For the correspondence between index values and TLB table entries, refer to the relevant section in TLB Maintenance Instructions.
    pub fn index(&self) -> usize {
        self.bits.get_bits(0..16)
    }
    /// When executing the TLBRD instruction, the value read from the PS field of the TLB table entry is recorded here.
    /// When executing the TLBWR and TLBFILL instructions with `CSR.TLBRERA.IsTLBR=0`,
    /// the value written to the PS field of the TLB table entry comes from here.
    pub fn ps(&self) -> usize {
        self.bits.get_bits(24..=29)
    }

    #[doc = "1 means the TLB table entry is empty (invalid TLB table entry),
and 0 means the TLB table entry is non-empty (valid TLB table entry)

* When executing the TLBSRCH instruction, this bit is recorded as 0 if there is a hit entry, otherwise it is recorded as 1.

* When executing the TLBRD instruction, the E bit read from the TLB table entry is inverted and recorded here.

* When executing the TLBWR instruction, then
  * If `CSR.TLBRFPC.IsTLBR`=0, the value written to the E bit of the TLB entry is written after it is inverted.
  * else, if `CSR.TLBRERA.IsTLBR`=1, then the E bit of the TLB entry being written is always set to 1, regardless of the value of that bit."]

    pub fn ne(&self) -> bool {
        self.bits.get_bit(31)
    }
}
/// See [`TlbIdx::index()`]
pub fn set_index(index: usize) {
    set_csr_loong_bits!(0x10, 0..16, index);
}

/// See [`TlbIdx::ps()`]
pub fn set_ps(ps: usize) {
    set_csr_loong_bits!(0x10, 24..=29, ps);
}

/// See [`TlbIdx::ne()`]
pub fn set_ne(ne: bool) {
    set_csr_loong_bit!(0x10, 31, ne);
}

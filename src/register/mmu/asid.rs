use bit_field::BitField;
impl_define_csr!(
    Asid,
    "Address Space ID\n\
     This register contains the ASID information for access operations and TLB-related instructions."
);

impl_read_csr!(0x18, Asid);

impl Asid {
    /// The ASID field is used to identify the address space of the virtual address.
    pub fn asid(&self) -> usize {
        self.bits.get_bits(0..10)
    }
    /// The ASID width
    pub fn asid_width(&self) -> usize {
        self.bits.get_bits(16..=23)
    }
}

pub fn set_asid(asid: usize) {
    set_csr_loong_bits!(0x18, 0..10, asid);
}

pub fn set_asid_width(asid_width: usize) {
    set_csr_loong_bits!(0x18, 16..=23, asid_width);
}

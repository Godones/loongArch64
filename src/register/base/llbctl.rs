use bit_field::BitField;
impl_define_csr!(
    Llbctl,
    "This register is used for the access control operations performed on the `LLBit`"
);

impl_read_csr!(0x60, Llbctl);

impl Llbctl {
    #[doc = "A read-only bit. Reading this bit will return the value of the current LLBit."]
    pub fn rollb(&self) -> bool {
        self.bits.get_bit(0)
    }
    /// Keep LLBit unclear once when `ERTN`, clearing `KLO` bit instead.
    pub fn klo(&self) -> bool {
        self.bits.get_bit(2)
    }
}
#[doc = "A software writing 1 to this bit will clear the LLBit to 0.
A software writing 0 to this bit will be *ignored* by hardware."]
pub fn set_wcclb(clear: bool) {
    set_csr_loong_bit!(0x60, 1, clear);
}
/// Set KLB to 1 to cancel hardware clear of `LLBit` from `ERTN` for only one time.
pub fn set_klo(ctr: bool) {
    set_csr_loong_bit!(0x60, 2, ctr);
}

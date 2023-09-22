use crate::register::CpuMode;
use bit_field::BitField;

impl_define_csr!(
    Prmd,
    "Pre-exception Mode Information (PRMD)
When an exception is triggered,
if the exception type is not TLB refill exception and machine error exception,
the hardware will save the processor core’s `PLV`,`IE` and `WE` bits at that time,
to `PRMD` to restore the processor core to the context when the exception returns.
"
);

impl_read_csr!(0x1, Prmd);

impl Prmd {
    /// In case of a non-TLB-refill and non-machine-error exception,
    /// the hardware records the old value of the `PLV` field in `CSR.CRMD` in this field.
    /// It later restores the value of this field to the PLV field of CSR.CRMD
    /// after `ERTN` instruction is executed to return from the exception handler.
    pub fn pplv(&self) -> usize {
        self.bits.get_bits(0..2)
    }
    /// Record the `CRMD.IE`(Interrupt Enable bit) before the non-TLB-refill and non-machine-error exception.
    pub fn pie(&self) -> bool {
        self.bits.get_bit(2)
    }
    /// In case of a non-TLB-refill and non-machine-error exception,
    /// the hardware records the old value of the `WE`(Watchpoint Enable bit) field in `CSR.CRMD` in this field.

    pub fn pwe(&self) -> bool {
        self.bits.get_bit(3)
    }
}

/// Set the value of this field to the PLV field of `CSR.CRMD` for later return through
/// `ERTN` instruction  from the exception handler and restoration of PLV.
///
/// See also [`Prmd::pplv`].
pub fn set_pplv(pplv: CpuMode) {
    set_csr_loong_bits!(0x1, 0..2, pplv as usize);
}
/// Set the value of this field to the `IE` field of `CSR.CRMD` for later return right after restoration of `IE` (Interrupt Enable bit),
///  from the exception handler through `ERTN` instruction.
///
/// See also [`Prmd::pie`].
pub fn set_pie(pie: bool) {
    set_csr_loong_bit!(0x1, 2, pie);
}

/// Set the value of this field to the `WE`(Watchpoint Enable bit) field of `CSR.CRMD` for later return from the exception handler,
/// through `ERTN` instruction and restoration of `WE`(Watchpoint Enable bit).

pub fn set_pwe(pwe: bool) {
    set_csr_loong_bit!(0x1, 3, pwe);
}

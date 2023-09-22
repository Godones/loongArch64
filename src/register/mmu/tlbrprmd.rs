use crate::register::CpuMode;
use bit_field::BitField;
use core::fmt::Debug;

impl_define_csr!(
    TlbRPrmd,
    "TLB Refill Exception Pre-exception Mode Information (TLBRPRMD)
When a TLB refill exception is triggered,
the hardware saves the processor core’s PLV, Guest mode, global IE, and WE into this register,
for restoration of the processor core accordingly when the exception returns.
"
);

impl_read_csr!(0x8f, TlbRPrmd);
impl Debug for TlbRPrmd {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TLBPrMd")
            .field("pplv", &self.pplv())
            .field("pie", &self.pie())
            .field("pwe", &self.pwe())
            .finish()
    }
}

impl TlbRPrmd {
    /// In case of TLB refill,
    /// the hardware records the old value of the `PLV` field in `CSR.CRMD` in this field.
    /// It later restores the value of this field to the PLV field of CSR.CRMD
    /// after `ERTN` instruction is executed to return from the exception handler.

    pub fn pplv(&self) -> usize {
        self.bits.get_bits(0..2)
    }
    /// Record the `CRMD.IE`(Interrupt Enable bit) before the TLB-refill exception.
    pub fn pie(&self) -> bool {
        self.bits.get_bit(2)
    }
    /// In case of a TLB refill exception,
    /// the hardware records the old value of the `WE`(Watchpoint Enable bit) field in `CSR.CRMD` in this field.
    pub fn pwe(&self) -> bool {
        self.bits.get_bit(4)
    }
}
/// Set the value of this field to the PLV field of `CSR.CRMD` for later return through
/// `ERTN` instruction  from the exception handler and restoration of PLV.

pub fn set_pplv(pplv: CpuMode) {
    set_csr_loong_bits!(0x8f, 0..2, pplv as usize);
}
/// Set the value of this field to the `IE` field of `CSR.CRMD` for later return right after restoration of `IE` (Interrupt Enable bit),
///  from the exception handler through `ERTN` instruction.

pub fn set_pie(pie: bool) {
    set_csr_loong_bit!(0x8f, 2, pie);
}
/// Set the value of this field to the `WE`(Watchpoint Enable bit) field of `CSR.CRMD` for later return from the exception handler,
/// through `ERTN` instruction and restoration of `WE`(Watchpoint Enable bit).

pub fn set_pwe(pwe: bool) {
    set_csr_loong_bit!(0x8f, 4, pwe);
}

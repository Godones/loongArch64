use super::estat::Interrupt;
use bit_field::BitField;
use bitflags::bitflags;
use core::fmt::Debug;
impl_define_csr!(Ecfg,"Exception Configuration (ECFG)
This register is used to control the entry calculation method of exceptions and interrupts and the local enable bit of each interrupt.");

impl_read_csr!(0x4, Ecfg);
bitflags! {
    #[derive(Debug,Copy,Clone)]
    pub struct LineBasedInterrupt:usize {
        ///Software Interrupt 0
        const SWI0=1<<Interrupt::SWI0 as usize;
        ///Software Interrupt 1
        const SWI1=1<<Interrupt::SWI1 as usize;
        ///Hardware Interrupt 0
        const HWI0=1<<Interrupt::HWI0 as usize;
        ///Hardware Interrupt 1
        const HWI1=1<<Interrupt::HWI1 as usize;
        ///Hardware Interrupt 2
        const HWI2=1<<Interrupt::HWI2 as usize;
        ///Hardware Interrupt 3
        const HWI3=1<<Interrupt::HWI3 as usize;
        ///Hardware Interrupt 4
        const HWI4=1<<Interrupt::HWI4 as usize;
        ///Hardware Interrupt 5
        const HWI5=1<<Interrupt::HWI5 as usize;
        ///Hardware Interrupt 6
        const HWI6=1<<Interrupt::HWI6 as usize;
        ///Hardware Interrupt 7
        const HWI7=1<<Interrupt::HWI7 as usize;
        ///Performance Monitor Counter Overflow Interrupt
        const PMCOV=1<<Interrupt::PMI as usize;
        ///Timer Interrupt
        const TIMER=1<<Interrupt::Timer as usize;
        ///Inter-Processor Interrupt
        const IPI=1<<Interrupt::IPI as usize;
    }
}

impl Ecfg {
    /// Returns the local interrupt status
    pub fn lie(&self) -> LineBasedInterrupt {
        LineBasedInterrupt::from_bits_truncate(self.bits.get_bits(0..13))
    }
    /// The `VS` field in `ECFG`.
    /// Configure the spacing of exceptions and interrupt entries.
    /// * When `VS`=0, all exceptions and interrupts have the same entry base address.
    /// * When `VS`!=0, the entry base address spacing between each exception and interrupt is `2VS` instructions.
    /// Since the TLB refill exceptions and machine error exceptions have separate entry base addresses,
    /// the entry of both exceptions is not affected by the `VS` field.
    pub fn vs(&self) -> usize {
        self.bits.get_bits(16..19)
    }
}

impl Debug for Ecfg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ECFG")
            .field("lie", &self.lie())
            .field("vs", &self.vs())
            .finish()
    }
}

pub fn set_vs(vs: usize) {
    set_csr_loong_bits!(0x4, 16..19, vs);
}

pub fn set_lie(lie: LineBasedInterrupt) {
    set_csr_loong_bits!(0x4, 0..13, lie.bits());
}

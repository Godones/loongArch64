use bit_field::BitField;
use core::fmt::Debug;
impl_define_csr!(Prcfg1, "Privileged Resource Configuration 1");
impl_read_csr!(0x21, Prcfg1);

impl Prcfg1 {
    /// The number of SAVE control and status registers.
    pub fn save_num(&self) -> usize {
        self.bits.get_bits(0..4)
    }
    /// The number of valid bit width of the timer.
    pub fn timer_bits(&self) -> usize {
        self.bits.get_bits(4..12) + 1
    }
    /// The maximum value that can be set for the exception and interrupt vector entry spacing (CSR.ECTL.VS).
    pub fn vs_max(&self) -> usize {
        self.bits.get_bits(12..15)
    }
}
impl Debug for Prcfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PRCfg1")
            .field("SAVE reg. number", &self.save_num())
            .field("Timer bits", &self.timer_bits())
            .field("max vector entry spacing", &self.vs_max())
            .finish()
    }
}

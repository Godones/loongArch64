use core::fmt::Debug;
impl_define_csr!(
    Tval,
    "Timer Value\n\
     The software can read this register to know the current count value of the timer.\n\
     The number of valid bits of the timer is determined by the implementation,\n\
     so the length of the TimeVal field in this register will also change."
);

impl_read_csr!(0x42, Tval);

impl Tval {
    pub fn time_val(&self) -> usize {
        self.bits
    }
}

impl Debug for Tval {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TVal").field("", &self.bits).finish()
    }
}

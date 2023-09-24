use bit_field::BitField;
use core::fmt::Debug;

impl_define_csr!(Tcfg, "Timer Configuration\n\
                        This register is the interface to the software configuration timer.\n\
                        The number of valid bits of the timer is determined by the implementation,\n\
                        so the length of the TimeVal field in this register will change accordingly.");
impl_read_csr!(0x41, Tcfg);

impl Debug for Tcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TCfg")
            .field("is_enabled", &self.en())
            .field("is_periodic", &self.periodic())
            .field("InitVal of (dec) timer", &self.init_val())
            .finish()
    }
}

impl Tcfg {
    /// Timer enable bit.
    /// Only when this bit is 1,
    /// the timer will perform countdown self decrement and set up the timing interrupt signal when it decrements to 0 value.
    pub fn en(&self) -> bool {
        !self.bits.get_bit(0)
    }
    /// Timer cycle mode control bit.
    /// If this bit is 1, when the timer decreases to 0,
    /// the timer will set up the timer interrupt signal and reload the timer to the initial value configured in the TimeVal field,
    /// and then continue to decrement itself in the next clock cycle.
    /// If this bit is 0, the timer will stop counting until the software configures the timer again when the countdown reaches 0.

    pub fn periodic(&self) -> bool {
        self.bits.get_bit(1)
    }
    /// The initial value of the timer countdown self decrement count.
    /// This initial value must be an integer multiple of 4.
    /// The hardware will automatically fill in the lowest bit of the field value.
    /// Two bits of 0 are added before it is used.

    pub fn init_val(&self) -> usize {
        (self.bits >> 2) << 2
    }
}
/// Only when this bit is 1,
/// the timer will perform countdown self decrement and set up the timing interrupt signal when it decrements to 0 value.

pub fn set_en(enable: bool) {
    set_csr_loong_bit!(0x41, 0, enable);
}
/// If this bit is 1, when the timer decreases to 0,
/// the timer will set up the timer interrupt signal and reload the timer to the initial value configured in the TimeVal field,
/// and then continue to decrement itself in the next clock cycle.
/// If this bit is 0, the timer will stop counting until the software configures the timer again when the countdown reaches 0.

pub fn set_periodic(loop_: bool) {
    set_csr_loong_bit!(0x41, 1, loop_);
}

/// Set the initial value of the timer countdown self decrement count.
/// The hardware will automatically fill in the lowest bit of the field value.
/// Two bits of 0 are added before it is used.
/// # Warning!
/// This initial value *MUST* be an integer multiple of 4.
pub fn set_init_val(val: usize) {
    set_csr_loong_bits!(0x41, 2.., val >> 2);
}

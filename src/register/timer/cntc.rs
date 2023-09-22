impl_define_csr!(Cntc, "Counter Compensation, \n\
                        This register can be configured by the software to correct the timer’s readout value.\n\
                        The final readout value will be the original `timer_count_val` + `timer_compensation`.\n\
                        It is important to note that configuring this register does not directly change the timer’s count value.\n\
                        In LA32, this register is 32-bit and its value will be sign extended to 64 bits and then added to the original counter value.");
impl_read_csr!(0x43, Cntc);

impl Cntc {
    /// Software-configurable counter compensation values.
    pub fn compensation(&self) -> usize {
        self.bits
    }
}
/// Set the software-configurable counter compensation values.
pub fn set_cntc(compensation: usize) {
    write_csr_loong!(0x43, compensation);
}

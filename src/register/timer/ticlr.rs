use bit_field::BitField;
impl_define_csr!(Ticlr,"Timer Interrupt Clearing \n\
                        The software clears the timed interrupt signal set by the timer by writing 1 to bit 0 of this register.");

impl_read_csr!(0x44, Ticlr);

impl Ticlr {}

pub fn clear_timer_interrupt() {
    set_csr_loong_bit!(0x44, 0, true);
}

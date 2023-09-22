use bit_field::BitField;
impl_define_csr!(Era, "Exception Return Address (ERA)\n\
                       Record the resulting PC in case of exceptions other than TLB Refill and Machine Error.");

impl_read_csr!(0x6, Era);

impl Era {
    pub fn pc(&self) -> usize {
        self.bits
    }
}

pub fn set_pc(pc: usize) {
    set_csr_loong_bits!(0x6, 0.., pc);
}

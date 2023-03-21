
use bit_field::BitField;
impl_define_csr!(Prcfg1);
impl_read_csr!(0x21,Prcfg1);


impl Prcfg1 {
    pub fn save_num(&self) -> usize {
        self.bits.get_bits(0..4)
    }
    pub fn timer_bits(&self) -> usize {
        // 返回定时器的位数
        self.bits.get_bits(4..12) + 1
    }
    pub fn vs_max(&self) -> usize {
        self.bits.get_bits(12..15)
    }
}

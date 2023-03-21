
use bit_field::BitField;
impl_define_csr!(Ecfg);
impl_write_csr!(0x4,Ecfg);
impl_read_csr!(0x4,Ecfg);

impl Ecfg {
    pub fn bits(&self) -> usize {
        self.bits
    }
    pub fn set_bits(&mut self, bits: usize) -> &mut Self {
        self.bits = bits;
        self
    }
    pub fn lie_with_index(&self, index: usize) -> bool {
        // 中断位于0-12位,每一位代表一个局部中断
        assert!(index < 13);
        self.bits.get_bit(index)
    }
    pub fn set_lie_with_index(&mut self, index: usize, val: bool) -> &mut Self {
        // 中断位于0-12位,每一位代表一个局部中断
        assert!(index < 13);
        self.bits.set_bit(index, val);
        self
    }
    // 例外处理中断入口的间距
    // 16-18位
    // 当此值为0 时，例外处理中断入口是同一个地址
    // 不为0时，每个异常有自己的中断入口
    pub fn vs(&self) -> usize {
        self.bits.get_bits(16..19)
    }
    pub fn set_vs(&mut self, value: usize) -> &mut Self {
        self.bits.set_bits(16..19, value);
        self
    }
}

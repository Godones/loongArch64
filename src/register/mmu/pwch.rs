use bit_field::BitField;
impl_define_csr!(Pwch);
impl_read_csr!(0x1d,Pwch);
impl_write_csr!(0x1d,Pwch);


impl Pwch {
    //次高一级目录的起始地址
    pub fn dir3_base(&self) -> usize {
        self.bits.get_bits(0..=5)
    }
    pub fn set_dir3_base(&mut self, dir2_base: usize) -> &mut Self {
        self.bits.set_bits(0..=5, dir2_base);
        self
    }
    // 次高一级目录的索引位数。0 表示没有这一级。
    pub fn dir3_width(&self) -> usize {
        self.bits.get_bits(6..=11)
    }
    pub fn set_dir3_width(&mut self, dir2_width: usize) -> &mut Self {
        self.bits.set_bits(6..=11, dir2_width);
        self
    }
    pub fn dir4_base(&self) -> usize {
        self.bits.get_bits(12..=17)
    }
    pub fn set_dir4_base(&mut self, dir3_base: usize) -> &mut Self {
        self.bits.set_bits(12..=17, dir3_base);
        self
    }
    pub fn dir4_width(&self) -> usize {
        self.bits.get_bits(18..=23)
    }
    pub fn set_dir4_width(&mut self, dir3_width: usize) -> &mut Self {
        self.bits.set_bits(18..=23, dir3_width);
        self
    }
}

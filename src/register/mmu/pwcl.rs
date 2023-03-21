use bit_field::BitField;
impl_define_csr!(Pwcl);
impl_read_csr!(0x1c,Pwcl);
impl_write_csr!(0x1c,Pwcl);


impl Pwcl {
    // 末级页表的起始地址。
    pub fn ptbase(&self) -> usize {
        self.bits.get_bits(0..=4)
    }
    pub fn set_ptbase(&mut self, ptbase: usize) -> &mut Self {
        self.bits.set_bits(0..=4, ptbase);
        self
    }
    // 末级页表的索引位数
    pub fn ptwidth(&self) -> usize {
        self.bits.get_bits(5..=9)
    }
    pub fn set_ptwidth(&mut self, ptwidth: usize) -> &mut Self {
        self.bits.set_bits(5..=9, ptwidth);
        self
    }
    pub fn dir1_base(&self) -> usize {
        self.bits.get_bits(10..=14)
    }
    pub fn set_dir1_base(&mut self, dir1_base: usize) -> &mut Self {
        self.bits.set_bits(10..=14, dir1_base);
        self
    }
    // 最低一级目录的索引位数。0 表示没有这一级
    pub fn dir1_width(&self) -> usize {
        self.bits.get_bits(15..=19)
    }
    pub fn set_dir1_width(&mut self, dir1_width: usize) -> &mut Self {
        self.bits.set_bits(15..=19, dir1_width);
        self
    }
    pub fn dir2_base(&self) -> usize {
        self.bits.get_bits(20..=24)
    }
    pub fn set_dir2_base(&mut self, dir2_base: usize) -> &mut Self {
        self.bits.set_bits(20..=24, dir2_base);
        self
    }
    // 最低两级目录的索引位数。0 表示没有这一级
    pub fn dir2_width(&self) -> usize {
        self.bits.get_bits(25..=29)
    }
    pub fn set_dir2_width(&mut self, dir2_width: usize) -> &mut Self {
        self.bits.set_bits(25..=29, dir2_width);
        self
    }
    // 0 表示 64 比特，1 表示 128 比特，2 表示 192 比特，3 表示 256 比特。
    pub fn pte_width(&self) -> usize {
        let val = self.bits.get_bits(30..=31);
        match val {
            0 => 64 / 8,
            1 => 128 / 8,
            2 => 192 / 8,
            3 => 256 / 8,
            _ => panic!("invalid pte_width"),
        }
    }
    pub fn set_pte_width(&mut self, pte_width: usize) -> &mut Self {
        let val = match pte_width {
            8 => 0,
            16 => 1,
            24 => 2,
            32 => 3,
            _ => panic!("invalid pte_width"),
        };
        self.bits.set_bits(30..=31, val);
        self
    }
}
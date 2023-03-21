use bit_field::BitField;

impl_define_csr!(Tcfg);
impl_read_csr!(0x41,Tcfg);
impl_write_csr!(0x41,Tcfg);


impl Tcfg {
    pub fn en(&self) -> bool {
        //第0位
        !self.bits.get_bit(0)
    }
    pub fn periodic(&self) -> bool {
        //第1位
        self.bits.get_bit(1)
    }
    pub fn init_val(&self) -> usize {
        //第2位开始
        (self.bits >> 2) << 2
    }
    pub fn bits(&self) -> usize {
        self.bits
    }
    pub fn set_bits(&mut self, val: usize) -> &mut Self {
        self.bits = val;
        self
    }
    pub fn set_en(&mut self, enable: bool) -> &mut Self {
        self.bits.set_bit(0, enable);
        self
    }
    pub fn set_periodic(&mut self, loop_: bool) -> &mut Self {
        self.bits.set_bit(1, loop_);
        self
    }
    pub fn set_init_val(&mut self, val: usize) -> &mut Self {
        // 设置计数值, 只能是4的整数倍
        // 在数值末尾会补上2bit0
        self.bits.set_bits(2.., val >> 2);
        self
    }
}

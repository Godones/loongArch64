use core::ops::Mul;
use bit_field::BitField;

impl_define_csr!(Misc);

impl Misc {
    pub fn va32l1(&self) -> bool {
        self.bits.get_bit(1)
    }
    pub fn set_va32l1(&mut self, value: bool) -> &mut Self {
        self.bits.set_bit(1, value);
        self
    }
    pub fn va32l2(&self)->bool{
        self.bits.get_bit(2)
    }
    pub fn set_va32l2(&mut self, value: bool) -> &mut Self {
        self.bits.set_bit(2, value);
        self
    }

    pub fn va32l3(&self) -> bool {
        self.bits.get_bit(3)
    }
    pub fn set_va32l3(&mut self, value: bool) -> &mut Self {
        self.bits.set_bit(3, value);
        self
    }

    pub fn drdtl1(&self) -> bool {
        self.bits.get_bit(5)
    }
    pub fn set_drdtl1(&mut self, value: bool) -> &mut Self {
        self.bits.set_bit(5, value);
        self
    }
    pub fn drdtl2(&self)->bool{
        self.bits.get_bit(6)
    }
    pub fn set_drdtl2(&mut self, value: bool) -> &mut Self {
        self.bits.set_bit(6, value);
        self
    }
    pub fn drdtl3(&self)->bool{
        self.bits.get_bit(7)
    }
    pub fn set_drdtl3(&mut self, value: bool) -> &mut Self {
        self.bits.set_bit(7, value);
        self
    }

    pub fn rpcntl1(&self) -> bool {
        self.bits.get_bit(9)
    }
    pub fn set_rpcntl1(&mut self, value: bool) -> &mut Self {
        self.bits.set_bit(9, value);
        self
    }
    pub fn rpcntl2(&self)->bool{
        self.bits.get_bit(10)
    }
    pub fn set_rpcntl2(&mut self, value: bool) -> &mut Self {
        self.bits.set_bit(10, value);
        self
    }
    pub fn rpcntl3(&self)->bool{
        self.bits.get_bit(11)
    }
    pub fn set_rpctl3(&mut self, value: bool) -> &mut Self {
        self.bits.set_bit(11, value);
        self
    }

    pub fn alcl0(&self)->bool{
        self.bits.get_bit(12)
    }
    pub fn set_alcl0(&mut self, value: bool) -> &mut Self {
        self.bits.set_bit(12, value);
        self
    }
    pub fn alcl1(&self)->bool{
        self.bits.get_bit(13)
    }
    pub fn set_alcl1(&mut self, value: bool) -> &mut Self {
        self.bits.set_bit(13, value);
        self
    }
    pub fn alcl2(&self)->bool{
        self.bits.get_bit(14)
    }
    pub fn set_alcl2(&mut self, value: bool) -> &mut Self {
        self.bits.set_bit(14, value);
        self
    }
    pub fn alcl3(&self)->bool{
        self.bits.get_bit(15)
    }
    pub fn set_alcl3(& mut self, value: bool) -> &mut Self {
        self.bits.set_bit(15, value);
        self
    }

    pub fn dwpl0(&self)->bool{
        self.bits.get_bit(16)
    }
    pub fn set_dwpl0(&mut self, value: bool) -> &mut Self {
        self.bits.set_bit(16, value);
        self
    }
    pub fn dwpl1(&self)->bool{
        self.bits.get_bit(17)
    }
    pub fn set_dwpl1(&mut self, value: bool) -> &mut Self {
        self.bits.set_bit(17, value);
        self
    }
    pub fn dwpl2(&self)->bool{
        self.bits.get_bit(18)
    }
    pub fn set_dwpl2(&mut self, value: bool) -> &mut Self {
        self.bits.set_bit(18, value);
        self
    }

}

impl_write_csr!(0x3,Misc);
impl_read_csr!(0x3,Misc);
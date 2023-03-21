
use bit_field::BitField;
impl_define_csr!(Llbctl);
impl_read_csr!(0x60,Llbctl);
impl_write_csr!(0x60,Llbctl);


impl Llbctl{
    pub fn rollb(&self)->bool{
        self.bits.get_bit(0)
    }

    pub fn set_wcllb(&mut self,clear:bool)->&mut Self{
        self.bits.set_bit(1,clear);
        self
    }
    pub fn klo(&self)->bool{
        self.bits.get_bit(2)
    }
    pub fn set_klo(&mut self,ctr:bool)->&mut Self{
        self.bits.set_bit(2,ctr);
        self
    }

}
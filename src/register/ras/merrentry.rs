use bit_field::BitField;
use crate::PALEN;
impl_define_csr!(MerrEntry);
impl_write_csr!(0x93,MerrEntry);
impl_read_csr!(0x93,MerrEntry);


impl MerrEntry{
    pub fn addr(&self)->usize{
        self.bits
    }
    pub fn set_addr(&mut self, addr: usize)->&mut Self{
        assert_eq!(addr & 0xFFF, 0);
        self.bits.set_bits(0..PALEN, addr);
        self
    }
}
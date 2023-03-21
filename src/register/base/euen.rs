
use bit_field::BitField;
impl_define_csr!(Euen);


impl Euen{
    pub fn bits(&self) -> usize {
        self.bits
    }
    pub fn fpe(& self,) ->bool {
        self.bits.get_bit(0)
    }
    pub fn set_fpe(&mut self, fpe: bool) -> &mut Self {
        self.bits.set_bit(0, fpe);
        self
    }
    pub fn sxe(& self,) ->bool {
        self.bits.get_bit(1)
    }
    pub fn set_sxe(&mut self, sxe: bool) -> &mut Self {
        self.bits.set_bit(1, sxe);
        self
    }
    pub fn asxe(& self,) ->bool {
        self.bits.get_bit(2)
    }
    pub fn set_asxe(&mut self, asxe: bool) -> &mut Self {
        self.bits.set_bit(2, asxe);
        self
    }
    pub fn bte(& self,) ->bool {
        self.bits.get_bit(3)
    }
    pub fn set_bte(&mut self, bte: bool) -> &mut Self {
        self.bits.set_bit(3, bte);
        self
    }
}
impl_write_csr!(0x2,Euen);
impl_read_csr!(0x2,Euen);
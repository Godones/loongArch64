use bit_field::BitField;
use crate::register::mmu::dmw::Dmw;
impl_define_csr!(Dmw3);
impl_read_csr!(0x183,Dmw3);
impl_write_csr!(0x183,Dmw3);

impl Dmw for Dmw3{
    fn plv0(&self) -> bool {
        self.bits.get_bit(0)
    }

    fn set_plv0(&mut self, plv0: bool) -> &mut Self {
        self.bits.set_bit(0, plv0);
        self
    }

    fn plv1(&self) -> bool {
        self.bits.get_bit(1)
    }

    fn set_plv1(&mut self, plv1: bool) -> &mut Self {
        self.bits.set_bit(1,plv1);
        self
    }

    fn plv2(&self) -> bool {
        self.bits.get_bit(2)
    }

    fn set_plv2(&mut self, plv2: bool) -> &mut Self {
        self.bits.set_bit(2,plv2);
        self
    }

    fn plv3(&self) -> bool {
        self.bits.get_bit(3)
    }

    fn set_plv3(&mut self, plv3: bool) -> &mut Self {
        self.bits.set_bit(3,plv3);
        self
    }

    fn mat(&self) -> usize {
        self.bits.get_bits(4..=5)
    }

    fn set_mat(&mut self, mat: usize) -> &mut Self {
        self.bits.set_bits(4..=5, mat);
        self
    }

    fn vseg(&self) -> usize {
        self.bits.get_bits(60..=63 )
    }

    fn set_vesg(&mut self, vseg: usize) -> &mut Self {
        self.bits.set_bits(60..=63, vseg);
        self
    }
}
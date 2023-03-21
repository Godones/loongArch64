use core::fmt;
use core::fmt::{Display};
use bit_field::BitField;
use crate::PALEN;
use crate::register::mmu::tlbelo::TLBEL;

impl_define_csr!(TlbElo1);
impl_read_csr!(0x13,TlbElo1);
impl_write_csr!(0x13,TlbElo1);

impl Display for TlbElo1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TlbElo1: RPLV:{},NX:{},NR:{},PPN:{:#x},G:{},MAT:{},PLV:{},D:{},V:{}",
            self.rplv(),
            self.not_executable(),
            self.not_readable(),
            self.ppn(),
            self.global(),
            self.mat(),
            self.plv(),
            self.dirty(),
            self.valid()
        )
    }
}
impl TLBEL for TlbElo1 {
    // 页表项的有效位（V）
    fn valid(&self) -> bool {
        self.bits.get_bit(0)
    }

    fn set_valid(&mut self, valid: bool) -> &mut Self {
        self.bits.set_bit(0, valid);
        self
    }

    fn dirty(&self) -> bool {
        self.bits.get_bit(1)
    }

    fn set_dirty(&mut self, dirty: bool) -> &mut Self {
        self.bits.set_bit(1, dirty);
        self
    }

    fn plv(&self) -> usize {
        self.bits.get_bits(2..=3)
    }

    fn set_plv(&mut self, plv: usize) -> &mut Self {
        self.bits.set_bits(2..=3, plv);
        self
    }

    fn mat(&self) -> usize {
        self.bits.get_bits(4..=5)
    }

    fn set_mat(&mut self, mem_access_type: usize) -> &mut Self {
        self.bits.set_bits(4..=5, mem_access_type);
        self
    }

    fn global(&self) -> bool {
        self.bits.get_bit(6)
    }

    fn set_global(&mut self, global_flag: bool) -> &mut Self {
        self.bits.set_bit(6, global_flag);
        self
    }

    fn ppn(&self, ) -> usize {
        self.bits.get_bits(14..PALEN)
    }

    fn set_ppn(&mut self, palen: usize, ppn: usize) -> &mut Self {
        self.bits.set_bits(14..palen, ppn);
        self
    }

    fn not_readable(&self) -> bool {
        self.bits.get_bit(61)
    }

    fn set_not_readable(&mut self, not_readable: bool) -> &mut Self {
        self.bits.set_bit(61, not_readable);
        self
    }

    fn not_executable(&self) -> bool {
        self.bits.get_bit(62)
    }

    fn set_not_executable(&mut self, not_executable: bool) -> &mut Self {
        self.bits.set_bit(62, not_executable);
        self
    }

    fn rplv(&self) -> bool {
        self.bits.get_bit(63)
    }

    fn set_rplv(&mut self, rplv: bool) -> &mut Self {
        self.bits.set_bit(63, rplv);
        self
    }
}
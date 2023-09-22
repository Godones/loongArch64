use bit_field::BitField;
use core::fmt::Debug;
impl_read_csr!(0x1d, Pwch);
impl_define_csr!(Pwch, "Page Walk Controller for Higher Half Address Space (PWCH)
This register and the information in the `CSR.PWCL` register together define the page table structure used in the operating system.
This information will be used to instruct software or hardware to perform page table walking.
See Multi-level Page Table Structure Supported by page walking for an illustration of the page table structure and walking process.
");

impl Pwch {
    /// Get the starting address of the level 3 directory.
    pub fn dir3_base(&self) -> usize {
        self.bits.get_bits(0..=5)
    }

    /// Get the number of index bits of the level 3 directory. 0 means there is no such level.
    pub fn dir3_width(&self) -> usize {
        self.bits.get_bits(6..=11)
    }

    /// Get the starting address of the level 4 directory.
    pub fn dir4_base(&self) -> usize {
        self.bits.get_bits(12..=17)
    }

    /// Get the number of index bits of the level 4 directory. 0 means there is no such level.
    pub fn dir4_width(&self) -> usize {
        self.bits.get_bits(18..=23)
    }
}
/// Set the starting address of the level 3 directory.
pub fn set_dir3_base(val: usize) {
    set_csr_loong_bits!(0x1d, 0..=5, val);
}
/// Set the number of index bits of the level 3 directory. 0 means there is no such level.
pub fn set_dir3_width(val: usize) {
    set_csr_loong_bits!(0x1d, 6..=11, val);
}
/// Set the starting address of the level 4 directory.
pub fn set_dir4_base(val: usize) {
    set_csr_loong_bits!(0x1d, 12..=17, val);
}
/// Set the number of index bits of the level 4 directory. 0 means there is no such level.
pub fn set_dir4_width(val: usize) {
    set_csr_loong_bits!(0x1d, 18..=23, val);
}

impl Debug for Pwch {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PWCH")
            .field("dir3_base", &self.dir3_base())
            .field("dir3_width", &self.dir3_width())
            .field("dir4_base", &self.dir4_base())
            .field("dir4_width", &self.dir3_width())
            .finish()
    }
}

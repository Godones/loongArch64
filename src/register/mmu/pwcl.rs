use bit_field::BitField;

impl_read_csr!(0x1c, Pwcl);

impl_define_csr!(Pwcl, "Page Walk Controller for Lower Half Address Space (PWCL)

The information in this register and the CSR.PWCH register together define the page table structure used in the operating system.
This information will be used to instruct software or hardware to perform page table walking.
See Multi-level Page Table Structure Supported by page walking for an illustration of the page table structure and walking process.

In LA32, only `PWCL` is implemented , making it a must for PWCL register to contain all the information needed to describe the page table structure.

Thus the last page table and the lowest two levels of the directory starting at no more than 32 bits, a restriction that still exists in LA64.
");

impl Pwcl {
    /// Get the start address of the last page table.
    pub fn ptbase(&self) -> usize {
        self.bits.get_bits(0..=4)
    }
    /// Get the starting address of the lowest level directory.
    pub fn ptwidth(&self) -> usize {
        self.bits.get_bits(5..=9)
    }

    /// Get the starting address of the lowest level directory.
    pub fn dir1_base(&self) -> usize {
        self.bits.get_bits(10..=14)
    }

    /// Get the number of index bits of the lowest level directory. 0 means there is no such level.
    pub fn dir1_width(&self) -> usize {
        self.bits.get_bits(15..=19)
    }

    /// Get the starting address of the next level directory.
    pub fn dir2_base(&self) -> usize {
        self.bits.get_bits(20..=24)
    }

    /// Get the number of index bits of the next lowest level directory. 0 means there is no such level.
    pub fn dir2_width(&self) -> usize {
        self.bits.get_bits(25..=29)
    }

    /// Get the length of each page table entry in the memory. 0 - 64 bit; 1 - 128 bit; 2 - 256 bit; 3 - 512 bit.
    pub fn pte_width(&self) -> usize {
        let val = self.bits.get_bits(30..=31);
        match val {
            0 => 64 / 8,
            1 => 128 / 8,
            2 => 256 / 8,
            3 => 512 / 8,
            _ => panic!("invalid pte_width"),
        }
    }
}

/// Set the start address of the last page table.
pub fn set_ptbase(ptbase: usize) {
    set_csr_loong_bits!(0x1c, 0..=4, ptbase);
}
/// Set the starting address of the lowest level directory.
pub fn set_ptwidth(ptwidth: usize) {
    set_csr_loong_bits!(0x1c, 5..=9, ptwidth);
}
/// Set the starting address of the lowest level directory.
pub fn set_dir1_base(dir1_base: usize) {
    set_csr_loong_bits!(0x1c, 10..=14, dir1_base);
}
/// Set the number of index bits of the lowest level directory. 0 means there is no such level.
pub fn set_dir1_width(dir1_width: usize) {
    set_csr_loong_bits!(0x1c, 15..=19, dir1_width);
}
/// Set the starting address of the next level directory.
pub fn set_dir2_base(dir2_base: usize) {
    set_csr_loong_bits!(0x1c, 20..=24, dir2_base);
}

/// Set the number of index bits of the next lowest level directory. 0 means there is no such level.

pub fn set_dir2_width(dir2_width: usize) {
    set_csr_loong_bits!(0x1c, 25..=29, dir2_width);
}
// Set the length of each page table entry in the memory. 0 - 64 bit; 1 - 128 bit; 2 - 192 bit; 3 - 256 bit.
pub fn set_pte_width(pte_width: usize) {
    let val = match pte_width {
        8 => 0,
        16 => 1,
        24 => 2,
        32 => 3,
        _ => panic!("invalid pte_width"),
    };
    set_csr_loong_bits!(0x1c, 30..=31, val);
}

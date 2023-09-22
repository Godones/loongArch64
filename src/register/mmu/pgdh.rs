use bit_field::BitField;
impl_read_csr!(0x1a, Pgdh);

impl_define_csr!(Pgdh, "Page Global Directory Base Address for Higher Half Address Space\n\
                        This register is used to configure the base address of the global directory for the lower half address space.\n\
                        It is required that the base address of the global directory must be aligned to a 4KB bound address.\n\
                        This register also contains the information related to the PS and P fields in the TLB table entry when executing the TLB-related instructions.
");

impl Pgdh {
    /// The base address of the global directory in the lower half address space.
    /// By lower half address space, it means that the [VALEN-1] bit of the virtual address is equal to 0.
    pub fn base(&self) -> usize {
        self.bits
    }
}

/// Set the base *ADDRESS* of the global directory in the higher half address space.
/// # Warning!
/// The address MUST be 4K page aligned.

pub fn set_base(val: usize) {
    assert_eq!(val & 0xFFF, 0);
    set_csr_loong_bits!(0x1a, 0.., val);
}

impl core::fmt::Debug for Pgdh {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PGDH")
            .field("bits", &format_args!("{:X}", self.bits))
            .finish()
    }
}

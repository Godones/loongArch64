use bit_field::BitField;

impl_define_csr!(Misc,"Miscellaneous Controller (MISC)

This register contains a number of control bits for the operating behavior of the processor core at different privilege levels, including whether to enable 32-bit address mode, whether to allow partially privileged instructions at non-privileged levels, whether to enable address non-alignment check, and whether to enable page table write protection check.
");

impl_read_csr!(0x3, Misc);
impl Misc {
    /// Whether to enable 32-bit address mode at the PLV1 privilege level.
    pub fn va32l1(&self) -> bool {
        self.bits.get_bit(1)
    }
    /// Whether to enable 32-bit address mode at the PLV2 privilege level.
    pub fn va32l2(&self) -> bool {
        self.bits.get_bit(2)
    }
    /// Whether to enable 32-bit address mode at the PLV3 privilege level.
    pub fn va32l3(&self) -> bool {
        self.bits.get_bit(3)
    }

    /// Whether to disable RDTIME-like instructions at the PLV1 privilege level.
    pub fn drdtl1(&self) -> bool {
        self.bits.get_bit(5)
    }

    /// Whether to disable RDTIME-like instructions at the PLV2 privilege level.
    pub fn drdtl2(&self) -> bool {
        self.bits.get_bit(6)
    }

    /// Whether to disable RDTIME-like instructions at the PLV3 privilege level.
    pub fn drdtl3(&self) -> bool {
        self.bits.get_bit(7)
    }

    /// Whether to allow software reads of the performance counter at the PLV1 privilege level.
    pub fn rpcntl1(&self) -> bool {
        self.bits.get_bit(9)
    }

    /// Whether to allow software reads of the performance counter at the PLV2 privilege level.
    pub fn rpcntl2(&self) -> bool {
        self.bits.get_bit(10)
    }

    /// Whether to allow software reads of the performance counter at the PLV3 privilege level.
    pub fn rpcntl3(&self) -> bool {
        self.bits.get_bit(11)
    }

    /// Whether to perform a non-alignment check for non-vector load/store instructions that are
    /// allowed to be non-aligned at PLV0 privilege level.
    pub fn alcl0(&self) -> bool {
        self.bits.get_bit(12)
    }

    /// Whether to perform a non-alignment check for non-vector load/store instructions that are
    /// allowed to be non-aligned at PLV0 privilege level.
    pub fn alcl1(&self) -> bool {
        self.bits.get_bit(13)
    }

    /// Whether to perform a non-alignment check for non-vector load/store instructions that are
    /// allowed to be non-aligned at PLV2 privilege level.
    pub fn alcl2(&self) -> bool {
        self.bits.get_bit(14)
    }

    /// Whether to perform a non-alignment check for non-vector load/store instructions that are
    /// allowed to be non-aligned at PLV3 privilege level.
    pub fn alcl3(&self) -> bool {
        self.bits.get_bit(15)
    }

    /// Whether to disable the check of the page table entry write protection during TLB virtual and
    /// real address translation at the PLV0 privilege level.
    pub fn dwpl0(&self) -> bool {
        self.bits.get_bit(16)
    }

    /// Whether to disable the check of the page table entry write protection during TLB virtual and
    /// real address translation at the PLV1 privilege level.
    pub fn dwpl1(&self) -> bool {
        self.bits.get_bit(17)
    }

    /// Whether to disable the check of the page table entry write protection during TLB virtual and
    /// real address translation at the PLV2 privilege level.
    pub fn dwpl2(&self) -> bool {
        self.bits.get_bit(18)
    }
}
/// Set to 1 to enable 32-bit address mode at the PLV1 privilege level.
pub fn set_va32l1(value: bool) {
    set_csr_loong_bit!(0x3, 1, value);
}
/// Set to 1 to enable 32-bit address mode at the PLV2 privilege level.
pub fn set_va32l2(value: bool) {
    set_csr_loong_bit!(0x3, 2, value);
}
/// Set to 1 to enable 32-bit address mode at the PLV3 privilege level.
pub fn set_va32l3(value: bool) {
    set_csr_loong_bit!(0x3, 3, value);
}

/// Set this bit to 1, to *DISABLE* execution of an RDTIME-like instruction,
/// triggering an instruction privilege level error exception (IPE) at the PLV1 privilege level instead.
pub fn set_drdtl1(value: bool) {
    set_csr_loong_bit!(0x3, 5, value);
}

/// Set this bit to 1, to *DISABLE* execution of an RDTIME-like instruction,
/// triggering an instruction privilege level error exception (IPE) at the PLV2 privilege level instead.
pub fn set_drdtl2(value: bool) {
    set_csr_loong_bit!(0x3, 6, value);
}
/// Set this bit to 1, to *DISABLE* execution of an RDTIME-like instruction,
/// triggering an instruction privilege level error exception (IPE) at the PLV1 privilege level instead.
pub fn set_drdtl3(value: bool) {
    set_csr_loong_bit!(0x3, 7, value);
}
/// Set this bit to 1, to allow CSRRD access to any of the implemented performance counters in PLV1,
/// instead of triggering an instruction privilege level error exception (IPE), in PLV1

pub fn set_rpcntl1(value: bool) {
    set_csr_loong_bit!(0x3, 9, value);
}
/// Set this bit to 1, to allow CSRRD access to any of the implemented performance counters in PLV2,
/// instead of triggering an instruction privilege level error exception (IPE), in PLV2.

pub fn set_rpcntl2(value: bool) {
    set_csr_loong_bit!(0x3, 10, value);
}

/// Set this bit to 1, to allow CSRRD access to any of the implemented performance counters in PLV3,
/// instead of triggering an instruction privilege level error exception (IPE), in PLV3.

pub fn set_rpcntl3(value: bool) {
    set_csr_loong_bit!(0x3, 11, value);
}

/// Set this bit to 1 to indicate that the misalignment check is performed,
/// and an address alignment error exception is triggered if illegal, in PLV0.
///
/// This bit is read/write only if the  non-aligned addresses for non-vector load/store instructions is supported.
///
/// Otherwise, the bit is a read-only constant 1.

pub fn set_alcl0(value: bool) {
    set_csr_loong_bit!(0x3, 12, value);
}

/// Set this bit to 1 to indicate that the misalignment check is performed,
/// and an address alignment error exception is triggered if illegal, in PLV1.
///
/// This bit is read/write only if the  non-aligned addresses for non-vector load/store instructions is supported.
///
/// Otherwise, the bit is a read-only constant 1.

pub fn set_alcl1(value: bool) {
    set_csr_loong_bit!(0x3, 13, value);
}
/// Set this bit to 1 to indicate that the misalignment check is performed,
/// and an address alignment error exception is triggered if illegal, in PLV2.
///
/// This bit is read/write only if the  non-aligned addresses for non-vector load/store instructions is supported.
///
/// Otherwise, the bit is a read-only constant 1.

pub fn set_alcl2(value: bool) {
    set_csr_loong_bit!(0x3, 14, value);
}
/// Set this bit to 1 to indicate that the misalignment check is performed,
/// and an address alignment error exception is triggered if illegal in PLV3.
///
/// This bit is read/write only if the  non-aligned addresses for non-vector load/store instructions is supported.
///

pub fn set_alcl3(value: bool) {
    set_csr_loong_bit!(0x3, 15, value);
}
/// Set this bit to 1, to disable a page modification exception for store instructions on accessing a page table entry with D=0.
pub fn set_dwpl0(value: bool) {
    set_csr_loong_bit!(0x3, 16, value);
}
/// Set this bit to 1, to disable a page modification exception for store instructions on accessing a page table entry with D=0 in PLV1.
pub fn set_dwpl1(value: bool) {
    set_csr_loong_bit!(0x3, 17, value);
}
/// Set this bit to 1, to disable a page modification exception for store instructions on accessing a page table entry with D=0 in PLV2.
pub fn set_dwpl2(value: bool) {
    set_csr_loong_bit!(0x3, 18, value);
}

impl core::fmt::Debug for Misc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Misc")
            .field(
                "32-bit addr plv(1,2,3):",
                &format_args!("{},{},{}", self.va32l1(), self.va32l2(), self.va32l3()),
            )
            .field(
                "rdtime allowed for plv(1,2,3):",
                &format_args!("{},{},{}", self.drdtl1(), self.drdtl2(), self.drdtl3()),
            )
            .field(
                "Disable dirty bit check for plv(0,1,2):",
                &format_args!("{},{},{}", self.dwpl0(), self.dwpl1(), self.dwpl2(),),
            )
            .field(
                "Misalignment check for plv(0,1,2,4):",
                &format_args!(
                    "{},{},{},{}",
                    self.alcl0(),
                    self.alcl1(),
                    self.alcl2(),
                    self.alcl3(),
                ),
            )
            .finish()
    }
}

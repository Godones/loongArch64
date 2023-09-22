use crate::register::{CpuMode, MemoryAccessType};
use bit_field::BitField;
impl_define_csr!(
    Crmd,
    "Current Mode Information (CRMD)
The information in this register is used to determine the the processor core’s privilege level,
global interrupt enable bit, watchpoint enable bit, and address translation mode at that time.
"
);
impl_read_csr!(0x0, Crmd);

impl Crmd {
    /// Current privilege level. The legal value range is 0 to 3,
    ///
    /// where 0 is the highest privilege level and 3 is the lowest privilege level.
    ///
    /// When an exception is triggered, the hardware sets this field to 0 to jump to the highest privilege level.
    ///
    /// When the ERTN instruction is executed to return from the exception handler,
    ///
    /// Three potential sources of this value is described as follows:
    /// 1. if CSR.MERRCTL.IsMERR=1, the hardware restores the value of the PPLV field of CSR.MERRCTL here;
    /// 2. otherwise, if CSR.TLBRERA.IsTLBR=1, the hardware restores the value of the PPLV field of CSR.TLBRPRMD to here;
    /// 3. finally, if neither of the two previous values are 1, hardware restores the value of the PPLV field of CSR.PRMD to here.
    pub fn plv(&self) -> CpuMode {
        self.bits.get_bits(0..2).into()
    }

    /// True if the machine is globally interrupt enabled.
    /// Otherwise false.

    pub fn ie(&self) -> bool {
        self.bits.get_bit(2)
    }
    /// True if the machine has enabled direct address translation.
    pub fn da(&self) -> bool {
        self.bits.get_bit(3)
    }
    /// True if the machine has enabled page translation.
    pub fn pg(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// The memory access type (MAT) for fetch operations in direct address translation mode.
    ///
    /// The hardware sets this field to 0 when a machine error exception is triggered.
    ///
    /// When the execution of the ERTN instruction returns from the exception handler and CSR.MERRCTL.IsMERR=1,
    /// the hardware restores the value of the PDATF field of CSR.MERRCTL to here.
    pub fn datf(&self) -> MemoryAccessType {
        self.bits.get_bits(5..=6).into()
    }

    /// The Memory Access Type(MAT) for load and store operations when in direct address translation mode.
    /// The field is set to 0(Strongly-ordered UnCached (SUC)) in case of a machine error exception.
    /// If `ERTN` instruction returns from the exception handler, and `CSR.MERRCTL.IsMERR`=1,
    /// the hardware restores the value of the `PDATM` field of `CSR.MERRCTL` to here.
    pub fn datm(&self) -> MemoryAccessType {
        self.bits.get_bits(7..=8).into()
    }

    /// Instruction and data watchpoints enable bit, which is active high.
    /// The hardware sets the value of this field to 0 when an exception is triggered.
    /// When the ERTN instruction is executed to return from the exception handler.
    /// If `CSR.MERRCTL.IsMERR`=1, the hardware restores the PWE field of CSR.MERRCTL here;
    /// otherwise, if `CSR.TLBRERA.IsTLBR`=1, the hardware restores the PWE field of CSR.TLBRPRMD here;
    /// otherwise, the hardware restores the value of the PWE field of CSR.PRMD here.
    pub fn we(&self) -> bool {
        self.bits.get_bit(9)
    }
}

impl core::fmt::Debug for Crmd {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CrMd")
            .field("plv", &self.plv())
            .field("ie", &self.ie())
            .field("we", &self.we())
            .field("is_paging_md", &self.pg())
            .field("is_dir_acc", &self.da())
            .field("datf", &self.datf())
            .field("datm", &self.datm())
            .finish()
    }
}

/// Set current privilege level.
pub fn set_plv(mode: CpuMode) {
    let mode = mode as usize;
    debug_assert!(mode < 4);
    set_csr_loong_bits!(0x0, 0..=1, mode);
}

/// Set the global interrupt enable bit.
pub fn set_ie(enable: bool) {
    set_csr_loong_bit!(0x0, 2, enable);
}

/// Set the direct address translation mode.
pub fn set_da(da: bool) {
    set_csr_loong_bit!(0x0, 3, da);
}

/// Set the page translation mode.
pub fn set_pg(pg: bool) {
    set_csr_loong_bit!(0x0, 4, pg);
}

/// Set memory access type in direct access mode.
/// # Warning!
///
/// In the case of using software to handle TLB refill, when the software sets PG to 1,
/// it's a MUST to set the DATF field to 0b01(Coherent Cacheable, aka. CC) at the same time.
/// See also: `get_datf()`
pub fn set_datf(datf: MemoryAccessType) {
    set_csr_loong_bits!(0x0, 5..=6, datf as usize);
}

/// Set Memory Access Type (MAT) for load and store operations when in direct address translation mode.
/// # Warning!
/// For software TLB refill, when the software sets `PG` to `1`,
/// it's a MUST to set `DATM` to `0b01`(Coherent Cacheable, aka. CC) at the same time.
pub fn set_datm(datm: MemoryAccessType) {
    set_csr_loong_bits!(0x0, 7..=8, datm as usize);
}

/// Set instruction and data watchpoints enable bit.
pub fn set_we(we: bool) {
    set_csr_loong_bit!(0x0, 9, we);
}

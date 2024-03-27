use crate::register::ras::merrctl;
use crate::register::ras::merrctl::MachineError;
use crate::register::{ecfg, tlbrera};
use bit_field::BitField;

impl_define_csr!(
    Estat,
    "Record the status information of the exceptions,
including the first(`Ecode`) and second level(`EsubCode`) encoding of the triggered exceptions,
and the status of each interrupt."
);

impl_read_csr!(0x5, Estat);

impl Estat {
    /// Returns the local interrupt status
    pub fn is(&self) -> usize {
        self.bits.get_bits(0..=12)
    }
    /// Returns the first level encoding of the triggered exceptions.
    ///
    /// When it is a TLB reload exception or a machine error exception,
    /// this field remains unchanged; otherwise, the hardware will write
    /// the value defined in the Ecode column in Table 7-8 to this field according to the exception type.
    pub fn ecode(&self) -> usize {
        self.bits.get_bits(16..=21)
    }

    /// Returns the second level encoding of the triggered exceptions.
    pub fn esubcode(&self) -> usize {
        self.bits.get_bits(22..=30)
    }

    pub fn cause(&self) -> Trap {
        // 优先判断是否是重填异常
        let is_tlb_reload = tlbrera::read().is_tlbr();
        if is_tlb_reload {
            return Trap::Exception(Exception::TLBRFill);
        } else if merrctl::read().is_merr() {
            return Trap::MachineError(MachineError::CacheCheckError);
        }
        let ecode = self.ecode();
        if ecode == 0 {
            // 仅当 CSR.ECFG.VS=0 时，表示是中断
            let ecfg_vs = ecfg::read().vs();
            if ecfg_vs == 0 {
                // 读取中断位
                let ie = self.is();
                for index in (0..13).rev() {
                    if ie.get_bit(index) {
                        return Trap::Interrupt(Interrupt::from_usize(index));
                    }
                }
            }
            return Trap::Unknown;
        }
        let sub_ecode = self.esubcode();
        match ecode {
            0x1 => Trap::Exception(Exception::LoadPageFault), // load
            0x2 => Trap::Exception(Exception::StorePageFault), // store
            0x3 => Trap::Exception(Exception::FetchPageFault), //取指操作页面不存在
            0x4 => Trap::Exception(Exception::PageModifyFault), //页修改例外
            0x5 => Trap::Exception(Exception::PageNonReadableFault), //页不可读
            0x6 => Trap::Exception(Exception::PageNonExecutableFault), //页不可执行
            0x7 => Trap::Exception(Exception::PagePrivilegeIllegal), //页特权级不合规
            0x8 => {
                match sub_ecode {
                    0x0 => Trap::Exception(Exception::FetchInstructionAddressError), //取指地址错误
                    0x1 => Trap::Exception(Exception::MemoryAccessAddressError), //访存地址访问错误
                    _ => Trap::Unknown,
                }
            }
            0x9 => Trap::Exception(Exception::AddressNotAligned), //地址不对齐
            0xa => Trap::Exception(Exception::BoundsCheckFault),  //越界例外
            0xb => Trap::Exception(Exception::Syscall),           //系统调用
            0xc => Trap::Exception(Exception::Breakpoint),        //调试中断
            0xd => Trap::Exception(Exception::InstructionNotExist), //指令不合规
            0xe => Trap::Exception(Exception::InstructionPrivilegeIllegal), //指令特权级不合规
            0xf => Trap::Exception(Exception::FloatingPointUnavailable), //浮点处理器不可用
            _ => Trap::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Exception {
    /// This exception is triggered when the virtual address of a LOAD(i.e. `ld.{b,h,w,d}`) operation finds a match in the TLB with `V=0`.
    LoadPageFault,
    /// This exception is triggered when the virtual address of a STORE(i.e. `st.{b,h,w,d}`) operation finds a match in the TLB with `V=0`
    StorePageFault,
    /// This exception is triggered when the virtual address of an instruction fetching  operation finds a match in the TLB with `V=0`.
    FetchPageFault,
    /// the virtual address of a store operation matches a TLB entry with `V=1`, `D=0` and a permitted privilege.
    PageModifyFault,
    /// the virtual address of a load operation matches a TLB entry with `V=1`, `NR=1` and a permitted privilege.
    PageNonReadableFault,
    /// the virtual address of a fetch operation matches a TLB entry with `V=1`, `NX=1` and a permitted privilege.
    PageNonExecutableFault,
    /// The page privilege level is illegal.
    PagePrivilegeIllegal,
    /// This exception is triggered when the virtual address of an instruction fetching operation is illegal.
    FetchInstructionAddressError,
    /// This exception is triggered when the virtual address of a load or store operation is illegal.
    MemoryAccessAddressError,
    /// This exception is triggered when the virtual address of a load or store operation is not aligned.
    AddressNotAligned,
    BoundsCheckFault,
    /// system call
    Syscall = 0xB,
    /// debug breakpoint
    Breakpoint = 0xC,
    InstructionNotExist = 0xD,
    InstructionPrivilegeIllegal = 0xE,
    FloatingPointUnavailable = 0xF,
    TLBRFill,
}

/// The interrupt type.
#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum Interrupt {
    ///Software Interrupt 0
    SWI0 = 0,
    ///Software Interrupt 1
    SWI1,
    ///Hardware Interrupt 0
    HWI0,
    ///Hardware Interrupt 1
    HWI1,
    ///Hardware Interrupt 2
    HWI2,
    ///Hardware Interrupt 3
    HWI3,
    ///Hardware Interrupt 4
    HWI4,
    ///Hardware Interrupt 5
    HWI5,
    ///Hardware Interrupt 6
    HWI6,
    ///Hardware Interrupt 7
    HWI7,
    ///Performance Monitor Counter Overflow Interrupt
    PMI,
    ///Timer Interrupt
    Timer,
    ///Inter-Processor Interrupt
    IPI,
}

impl Interrupt {
    pub fn from_usize(value: usize) -> Self {
        match value {
            0 => Interrupt::SWI0,
            1 => Interrupt::SWI1,
            2 => Interrupt::HWI0,
            3 => Interrupt::HWI1,
            4 => Interrupt::HWI2,
            5 => Interrupt::HWI3,
            6 => Interrupt::HWI4,
            7 => Interrupt::HWI5,
            8 => Interrupt::HWI6,
            9 => Interrupt::HWI7,
            10 => Interrupt::PMI,
            11 => Interrupt::Timer,
            12 => Interrupt::IPI,
            _ => panic!("invalid interrupt index"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Trap {
    Exception(Exception),
    Interrupt(Interrupt),
    MachineError(MachineError),
    Unknown,
}

/// Set the software interrupt enable bit.
///
/// # Warning!
/// The index of software interrupt is 0 and 1.
pub fn set_sw(index: usize, value: bool) {
    set_csr_loong_bit!(0x5, index, value);
}

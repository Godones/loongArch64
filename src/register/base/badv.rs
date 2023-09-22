impl_define_csr!(Badv, "Bad Virtual Address (BADV)
This register is used to record the bad address when a bad address exception is triggered. Such exceptions include:
* ADdress error Exception for Fetching instructions (ADEF), at this time the PC of the instruction is recorded
* ADdress error Exception for Memory access instructions (ADEM)
* Address aLignment fault Exception (ALE)
* Bound Check Exception (BCE)
* Page Invalid exception for Load operation (PIL)
* Page Invalid exception for Store operation (PIS)
* Page Invalid exception for Fetch operation (PIF)
* Page Modification Exception (PME)
* Page Non-Readable exception (PNR)
* Page Non-eXecutable exception (PNX)
* Page Privilege level Illegal exception (PPI)");

impl_read_csr!(0x7, Badv);

impl Badv {
    /// Returns the bad address
    pub fn vaddr(&self) -> usize {
        self.bits
    }
}

pub fn write(vaddr: usize) {
    unsafe {
        core::arch::asm!("csrwr {},{}", in(reg) vaddr, const 0x7);
    }
}

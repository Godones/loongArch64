impl_define_csr!(TlbRSave, " TLB Refill Exception Data Save Register (TLBRSAVE)

This register is used to store data temporarily for the system software.
Each dava save register can hold the data of one general-purpose register.

The reason for the additional SAVE register for TLB refill exception processing is:
To address the case where a TLB refill exception is triggered during the processing of exceptions except the TLB refill exception.
");

impl_read_csr!(0x8b, TlbRSave);

impl TlbRSave {
    pub fn data(&self) -> usize {
        self.bits
    }
}

pub fn set_data(value: usize) {
    write_csr_loong!(0x8b, value);
}

use crate::register::MemoryAccessType;
use crate::PALEN;
use bit_field::BitField;
impl_read_csr!(0x8d, TlbRElo1);

impl_define_csr!(TlbRElo1,"TLB Refill Exception Entry Low-order Bits (TLBRELO0, TLBRELO1)

The TLBRELO registers store the low-order bits of PPN-related information in the TLB table entry,
during executing TLB-related instructions
(when the TLB refill exception context CSR.TLBRERA.IsTLBR=1).

The format of TLBRELO registers and the meaning of each field are the same as TLBELO registers.

However, the TLBRELO registers are not an exact copy of the TLBELO registers,
in the case of CSR.TLBRERA.IsTLBR=1. This is reflected in two points:

* Regardless of the value of CSR.TLBRERA.IsTLBR, the TLBRD instruction updates only the TLBELO0/TLBELO1 registers.

* Regardless of the value of CSR.TLBRERA.IsTLBR, the LDPTE instruction updates only the TLBRELO0/TLBRELO1 registers.
");

impl_tlbelo!(TlbRElo1, 0x8d);
impl core::fmt::Debug for TlbRElo1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TLBRELo1")
            .field("bits", &self.bits)
            .field("valid", &self.valid())
            .field("MAT", &self.mat())
            .field("NR", &self.not_readable())
            .field("NX", &self.not_executable())
            .field("ppn", &self.ppn())
            .field("dirty", &self.dirty())
            .field("rplv", &self.rplv())
            .field("global", &self.global())
            .field("plv", &self.plv())
            .finish()
    }
}

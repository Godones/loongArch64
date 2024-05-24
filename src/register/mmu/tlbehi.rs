use crate::VALEN;
use bit_field::BitField;
impl_read_csr!(0x11, TlbEhi);
impl_define_csr!(TlbEhi, "TLB Entry High-order Bits (TLBEHI)

This register contains the information related to the VPN of the high-order bits of the TLB table entry for TLB-related instructions.

Since the length of the `VPPN` field contained in the high-order bits of the TLB table entry is depends on the range of valid virtual addresses supported by the implementation,
the definition of the relevant register field is expressed separately.
");

impl TlbEhi {
    #[doc = "
* When executing the `TLBRD` instruction, the value of the `VPPN` field read from the `TLB` table entry is recorded here.

* When `CSR.TLBRERA.IsTLBR`=0, the VPPN value used to query `TLB` when executing `TLBSRCH` instruction and the value of VPPN field written to `TLB` table entry when executing `TLBWR` and `TLBFILL` instructions come from here.

* When the page invalid exception for load operation, page invalid exception for store operation, page invalid exception for fetch operation, page modification exception, page non-readable exception, page non-executable exception, and page privilege level ilegal exception are triggered, the 31:13 bits of the virual address that triggered the exception are recorded here.
"]

    pub fn vppn(&self) -> usize {
        self.bits.get_bits(13..VALEN)
    }
}
#[doc = "* When executing the `TLBRD` instruction, the value of the `VPPN` field read from the `TLB` table entry is recorded here.

* When `CSR.TLBRERA.IsTLBR`=0, the VPPN value used to query `TLB` when executing `TLBSRCH` instruction and the value of VPPN field written to `TLB` table entry when executing `TLBWR` and `TLBFILL` instructions come from here.

* When the page invalid exception for load operation, page invalid exception for store operation, page invalid exception for fetch operation, page modification exception, page non-readable exception, page non-executable exception, and page privilege level ilegal exception are triggered, the 31:13 bits of the virual address that triggered the exception are recorded here.
"]

pub fn set_vppn(_valen: usize, vppn: usize) {
    set_csr_loong_bits!(0x11, 13..VALEN, vppn);
}

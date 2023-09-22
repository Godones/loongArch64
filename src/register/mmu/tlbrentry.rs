impl_read_csr!(0x88, TlbREntry);
impl_define_csr!(
    TlbREntry,
    " TLB Refill Exception Entry Base Address (TLBRENTRY)

This register is used to configure the entry base address of the TLB refill exception.
Since the processor core enters direct address translation mode after triggering TLB refill exception,
the entry base address filled here should be a physical address.
"
);

impl TlbREntry {
    /// Get the TLB refill entry.
    pub fn addr(&self) -> usize {
        self.bits
    }
}
/// Set the TLB refill entry.
/// # Warning!
/// The `val` must be page aligned.

pub fn set_tlbrentry(addr: usize) {
    assert_eq!(addr & 0xFFF, 0);
    write_csr_loong!(0x88, addr);
}

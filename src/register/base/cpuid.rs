impl_define_csr!(
    CpuId,
    "This register contains the processor core number information."
);

impl_read_csr!(0x20, CpuId);

impl CpuId {
    /// Returns the core id
    pub fn core_id(&self) -> usize {
        self.bits
    }
}

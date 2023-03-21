
impl_define_csr!(Cpuid);
impl_read_csr!(0x20,Cpuid);



impl Cpuid {
    pub fn core_id(&self) -> usize {
        self.bits
    }
}

pub trait TLBEL {
    fn valid(&self) -> bool;
    fn set_valid(&mut self, valid: bool) -> &mut Self;
    fn dirty(&self) -> bool;
    fn set_dirty(&mut self, dirty: bool) -> &mut Self;
    fn plv(&self) -> usize;
    fn set_plv(&mut self, plv: usize) -> &mut Self;
    fn mat(&self) -> usize;
    fn set_mat(&mut self, mem_access_type: usize) -> &mut Self;
    fn global(&self) -> bool;
    fn set_global(&mut self, global_flag: bool) -> &mut Self;
    fn ppn(&self,) -> usize;
    fn set_ppn(&mut self, palen: usize, ppn: usize) -> &mut Self;
    fn not_readable(&self) -> bool;
    fn set_not_readable(&mut self, not_readable: bool) -> &mut Self;
    fn not_executable(&self) -> bool;
    fn set_not_executable(&mut self, not_executable: bool) -> &mut Self;
    fn rplv(&self) -> bool;
    fn set_rplv(&mut self, rplv: bool) -> &mut Self;
}






use crate::register::MemoryAccessType;
use bit_field::BitField;
use core::fmt::Debug;
impl_define_csr!(Dmw1, "Direct Mapping Configuration Window 1\n\
                       This group sender is involved in completing the direct mapping address translation mode.\n\
                       See Direct Mapped Address Translation Mode for more information about this address translation mode.");
impl_read_csr!(0x181, Dmw1);

impl_dwm!(Dmw1, 0x181);

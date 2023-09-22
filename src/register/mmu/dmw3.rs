use crate::register::MemoryAccessType;
use core::fmt::Debug;
use bit_field::BitField;
impl_define_csr!(Dwm3,"Direct Mapping Configuration Window 3\n\
                       This group sender is involved in completing the direct mapping address translation mode.\n\
                       See Direct Mapped Address Translation Mode for more information about this address translation mode.");
impl_read_csr!(0x183, Dwm3);
impl_dwm!(Dwm3,0x183);
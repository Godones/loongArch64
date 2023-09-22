use core::fmt::Debug;
use bit_field::BitField;
use crate::register::MemoryAccessType;
impl_define_csr!(Dmw0,"Direct Mapping Configuration Window 0\n\
                       This group sender is involved in completing the direct mapping address translation mode.\n\
                       See Direct Mapped Address Translation Mode for more information about this address translation mode.");
impl_read_csr!(0x180, Dmw0);

impl_dwm!(Dmw0,0x180);


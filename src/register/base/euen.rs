use bit_field::BitField;
impl_define_csr!(Euen,"Extended Component Unit Enable (EUEN)

In addition to the base integer instruction set and the privileged instruction set,
the base floating-point instruction set,
the binary translation extension instruction set,
the 128-bit vector extension instruction set,
and the 256-bit vector extension instruction set each have software-configurable enable bits.

When these enable controls are disabled, execution of the corresponding instruction will trigger the corresponding instruction unavailable exception.

Software uses this mechanism to determine the scope when saving the context.

Hardware implementations can also use the control bits here to implement circuit power control.
");

impl_read_csr!(0x2, Euen);
impl Euen {
    #[doc = "The base floating-point instruction enable bit.
If disabled, execution of the base floating-point instruction will trigger a floating-point instruction disable exception (FPD)."]
    pub fn fpe(&self) -> bool {
        self.bits.get_bit(0)
    }

    #[doc = "The base SIMD extension instruction enable bit.
If disabled, execution of the SIMD instruction will trigger an SIMD extension instruction disable exception (SXD)."]
    pub fn sxe(&self) -> bool {
        self.bits.get_bit(1)
    }
    #[doc = "The Advanced SIMD extension instruction enable bit.
If disabled, execution of the Advanced SIMD instruction will trigger an Advanced SIMD extension instruction disable exception (ASXD)."]
    pub fn asxe(&self) -> bool {
        self.bits.get_bit(2)
    }
    #[doc = "The Binary Translation extension instruction enable bit.
If disabled, execution of the Binary Translation instruction will trigger an Binary Translation extension instruction disable exception (BTD)."]
    pub fn bte(&self) -> bool {
        self.bits.get_bit(3)
    }
}

/// Set the base floating-point instruction enable bit.
pub fn set_fpe(fpe: bool) {
    set_csr_loong_bit!(0x2, 0, fpe);
}

/// Set the base SIMD extension instruction enable bit.
pub fn set_sxe(sxe: bool) {
    set_csr_loong_bit!(0x2, 1, sxe);
}

/// Set the Advanced SIMD extension instruction enable bit.
pub fn set_asxe(asxe: bool) {
    set_csr_loong_bit!(0x2, 2, asxe);
}

pub fn set_bte(bte: bool) {
    set_csr_loong_bit!(0x2, 3, bte);
}

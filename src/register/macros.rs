
macro_rules! read_csr_loong {
    ($csr_number:literal) => {
        #[inline]
        unsafe fn _read() -> usize {
            let bits:usize;
            core::arch::asm!("csrrd {},{}", out(reg) bits, const $csr_number);
            bits
        }
    };
}


macro_rules! impl_read_csr {
    ($csr_number:literal,$csr_ident:ident) => {
        read_csr_loong!($csr_number);
        #[inline]
        pub fn read() -> $csr_ident {
            $csr_ident { bits: unsafe{_read()} }
        }
    };
}


macro_rules! write_csr_loong {
    ($csr_number:literal) => {
        #[inline]
        unsafe fn _write(bits: usize) {
            core::arch::asm!("csrwr {},{}", in(reg) bits, const $csr_number);
        }
    };
}

macro_rules! impl_write_csr {
    ($csr_number:literal,$csr_ident:ident) => {
        write_csr_loong!($csr_number);
        impl $csr_ident{
            pub fn write(&mut self) {
                unsafe { _write(self.bits) }
            }
        }
    };
}

macro_rules! impl_define_csr {
    ($csr_ident:ident) => {
        #[derive(Debug,Copy,Clone)]
        pub struct $csr_ident {
            pub bits: usize,
        }
    };
}
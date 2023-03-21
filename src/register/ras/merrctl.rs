use bit_field::BitField;
impl_define_csr!(MerrCtl);
impl_write_csr!(0x90,MerrCtl);
impl_read_csr!(0x90,MerrCtl);


impl MerrCtl{
    pub fn is_merr(&self)->bool{
        self.bits.get_bit(0)
    }
    pub fn repairable(&self)->bool{
        self.bits.get_bit(1)
    }
    pub fn pplv(&self)->usize{
        self.bits.get_bits(2..=3) as usize
    }
    pub fn set_pplv(&mut self, pplv: usize)->&mut Self{
        self.bits.set_bits(2..=3, pplv);
        self
    }
    pub fn ie(&self)->bool{
        self.bits.get_bit(4)
    }
    pub fn set_id(&mut self, ie: bool)->&mut Self{
        self.bits.set_bit(4, ie);
        self
    }
    pub fn pwe(&self)->bool{
        self.bits.get_bit(6)
    }
    pub fn set_pwe(&mut self, pwe: bool)->&mut Self{
        self.bits.set_bit(6, pwe);
        self
    }
    pub fn pda(&self)->bool{
        self.bits.get_bit(7)
    }
    pub fn set_pda(&mut self, pda: bool)->&mut Self{
        self.bits.set_bit(7, pda);
        self
    }
    pub fn ppg(&self)->bool{
        self.bits.get_bit(8)
    }
    pub fn set_ppg(&mut self, ppg: bool)->&mut Self{
        self.bits.set_bit(8, ppg);
        self
    }
    pub fn pdatf(&self)->usize{
        self.bits.get_bits(9..=10)
    }
    pub fn set_pdatf(&mut self, pdatf: usize)->&mut Self{
        self.bits.set_bits(9..=10, pdatf);
        self
    }
    pub fn pdatm(&self)->usize{
        self.bits.get_bits(11..=12)
    }
    pub fn set_pdatm(&mut self, pdatm: usize)->&mut Self{
        self.bits.set_bits(11..=12, pdatm);
        self
    }
    pub fn cause(&self)->MachineError{
        let code = self.bits.get_bits(13..=15);
        MachineError::from(code)
    }

}

pub enum MachineError{
    CacheCheckError,
}

impl From<usize> for MachineError{
    fn from(cause: usize)->Self{
        match cause{
            0x1 => MachineError::CacheCheckError,
            _ => panic!("Unknown machine error"),
        }
    }
}
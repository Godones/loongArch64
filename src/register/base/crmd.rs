
use bit_field::BitField;
use crate::register::CpuMode;

// 当前模式信息
impl_define_csr!(Crmd);


impl Crmd {
    // 返回整个寄存器的内容
    pub fn bits(&self) -> usize {
        self.bits
    }
    pub fn set_bits(&mut self, bits: usize) -> &mut Self {
        self.bits = bits;
        self
    }
    // 返回当前特权级模式
    // 0-1位
    pub fn plv(&self) -> usize {
        self.bits.get_bits(0..2)
    }
    // 设置特权级模式
    pub fn set_plv(&mut self, mode: CpuMode) -> &mut Self {
        self.bits.set_bits(0..2, mode as usize);
        self
    }
    // 设置全局中断使能
    // 第2位
    pub fn set_ie(&mut self, enable: bool) -> &mut Self {
        self.bits.set_bit(2, enable);
        self
    }
    // 获取全局中断使能
    pub fn ie(&self) -> bool {
        self.bits.get_bit(2)
    }
    // 直接地址翻译
    pub fn da(&self) -> bool {
        // 第3位
        self.bits.get_bit(3)
    }
    // 设置直接地址翻译使能
    pub fn set_da(&mut self, da: bool) -> &mut Self {
        self.bits.set_bit(3, da);
        self
    }
    // 获取PG
    // 第4位
    pub fn pg(&self) -> bool {
        self.bits.get_bit(4)
    }
    // 设置PG,页翻译使能
    pub fn set_pg(&mut self, pg: bool) -> &mut Self {
        self.bits.set_bit(4, pg);
        self
    }
    // 获取直接地址翻译模式时，取指操作的存储访问类型
    // 在采用软件处理 TLB 重填的情况下，当软件将 PG 置为 1 时，需同时将 DATF 域置为
    // 0b01，即一致可缓存类型
    pub fn datf(&self) -> usize {
        self.bits.get_bits(5..=6)
    }
    pub fn set_datf(&mut self, datf: usize) -> &mut Self {
        self.bits.set_bits(5..=6, datf);
        self
    }
    // 直接地址翻译模式时，load 和 store 操作的存储访问类型
    pub fn datm(&self) -> usize {
        self.bits.get_bits(7..=8)
    }
    pub fn set_datm(&mut self, datm: usize) -> &mut Self {
        self.bits.set_bits(7..=8, datm);
        self
    }
    // 指令和数据监视点的使能位
    pub fn we(&self) -> bool {
        // 第9位
        self.bits.get_bit(9)
    }
    pub fn set_we(&mut self,we:bool)->&mut Self{
        self.bits.set_bit(9,we);
        self
    }
}

impl_write_csr!(0x0,Crmd);
impl_read_csr!(0x0,Crmd);

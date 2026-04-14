use loongArch64::register;
use loongArch64::register::MemoryAccessType;

fn main() {
    let crmd = register::crmd::read();
    let _datm = crmd.datm();
    register::crmd::set_datm(MemoryAccessType::CoherentCached);
}

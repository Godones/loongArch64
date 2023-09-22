use loongarch64::register;
use loongarch64::register::MemoryAccessType;

fn main() {
    let crmd = register::crmd::read();
    let _datm = crmd.datm();
    register::crmd::set_datm(MemoryAccessType::CoherentCached);
}

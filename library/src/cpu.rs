pub trait Cpu {
    fn execute(&mut self) -> u32;
    fn ime(&self) -> bool;
    fn int(&mut self, addr: u16);
}

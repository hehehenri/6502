pub trait Bus {
    fn read_u8(&self, address: u16) -> u8;
    fn write_u8(&mut self, address: u16, value: u8);
}

const SIZE: usize = 1024 * 64;

pub struct Memory([u8; SIZE]);

impl Memory {
    pub fn empty() -> Self {
        Memory([0x00; SIZE])
    }
}

impl Bus for Memory {
    fn read_u8(&self, address: u16) -> u8 {
        self.0[address as usize]
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        self.0[address as usize] = value;
    }
}

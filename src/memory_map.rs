use std::boxed::Box;

pub struct MemoryMap {
    //mem: box [u8; 0xFFFF],
    pub mem: Box<[u8; 0xFFFF]>,
}

impl MemoryMap {
    // ctor returning a self pointer
    pub fn new() -> Self {
        Self {
            mem: Box::new([0; 0xFFFF]),
        }
    }

    pub fn load_byte(&mut self, address: usize, data: u8) {
        self.mem[address] = data;
    }

    pub fn load_vec(&mut self, address: usize, data: Vec<u8>) {
        for (address, data) in self.mem.iter_mut().zip(data.iter()) {
            *address = *data
        }
    }

    pub fn dump(&self) {
        println!("dump()");

        for i in 0..500 {
            println!("{:x?}", self.mem[i]);
        }
    }
}

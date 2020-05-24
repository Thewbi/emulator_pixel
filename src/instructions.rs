use super::cpu::CPU;
use super::instruction_targets::LoadWordTarget;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub struct LD_N_NN {
    pub opcode: u8,
    pub cycles: u8,
    pub target: LoadWordTarget,
    pub data1: u8,
    pub data2: u8,
}

impl LD_N_NN {
    // ctor
    pub fn new(_opcode: u8, _cycles: u8, _target: LoadWordTarget, _data1: u8, _data2: u8) -> Self {
        Self {
            opcode: _opcode,
            cycles: _cycles,
            target: _target,
            data1: _data1,
            data2: _data2,
        }
    }
}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub struct XOR {
    opcode: u8,
    cycles: u8,
    data: u8,
}

impl XOR {
    // ctor
    pub fn new(_opcode: u8, _cycles: u8, _data: u8) -> Self {
        Self {
            opcode: _opcode,
            cycles: _cycles,
            data: _data,
        }
    }
}

pub struct Nop {}

impl Nop {}

pub trait Instruction {
    fn execute(&self, _cpu: &mut CPU);
}

// 3.3.2. 16-Bit Loads, p.76
impl Instruction for LD_N_NN {
    fn execute(&self, _cpu: &mut CPU) {
        // DEBUG
        println!(
            "LD_N_NN target: {0:?}, data1: {1}, data2: {2}",
            self.target, self.data1, self.data2
        );

        match self.opcode {
            0x01 => {
                _cpu.regs.b = self.data1;
                _cpu.regs.c = self.data2;
            }
            0x11 => {
                _cpu.regs.d = self.data1;
                _cpu.regs.e = self.data2;
            }
            0x21 => {
                _cpu.regs.h = self.data1;
                _cpu.regs.l = self.data2;
            }
            0x31 => {
                _cpu.sp = (self.data2 as u16) << 8 | self.data1 as u16;
            }
            _ => {}
        }
    }
}

// 7. XOR n, p.86
impl Instruction for XOR {
    fn execute(&self, _cpu: &mut CPU) {
        // TODO: implement operation
        // TODO: update flags register
        match self.opcode {
            0xAF => {}
            0xA8 => {}
            0xA9 => {}
            0xAA => {}
            0xAB => {}
            0xAC => {}
            0xAD => {}
            0xAE => {}
            0xEE => {}
            _ => {}
        }
    }
}

impl Instruction for Nop {
    fn execute(&self, _cpu: &mut CPU) {
        println!("Nop");
    }
}

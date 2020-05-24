use super::cpu;
use super::instruction_targets;
use super::instructions;

use std::boxed::Box;
use std::panic;

#[allow(dead_code)]
pub struct InstructionBuilder {}

#[allow(dead_code)]
impl InstructionBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create(&mut self, _cpu: &mut cpu::CPU) -> Box<dyn instructions::Instruction> {
        let _nb = _cpu.next();
        println!("next_byte: 0x{:x?}", _nb);

        match _nb {
            // 16 bit load
            0x01 | 0x11 | 0x21 | 0x31 => Box::new(instructions::LD_N_NN::new(
                _nb,
                12,
                instruction_targets::from_load_word_target(_nb),
                _cpu.next(),
                _cpu.next(),
            )),
            // xor
            0xA8 | 0xA9 | 0xAA | 0xAB | 0xAC | 0xAD | 0xAE | 0xAF | 0xEE => {
                let cycles = if _nb == 0xAE | 0xEE { 8 } else { 4 };
                Box::new(instructions::XOR::new(_nb, cycles, _cpu.next()))
            }
            // default
            _ => {
                panic!("Unknown instruction: 0x{:x}", _nb);
            }
        }
    }
}

//use super::instruction_builder;
use super::memory_map;
use super::registers;

#[allow(dead_code)]
pub struct CPU {
    pub regs: registers::Registers,
    mmap: memory_map::MemoryMap,
    pub pc: u16,
    pub sp: u16,
    //pub bus: MemoryBus,
    is_halted: bool,
    interrupts_enabled: bool,
}

#[allow(dead_code)]
impl CPU {
    // ctor
    pub fn new(_mmap: memory_map::MemoryMap) -> Self {
        Self {
            regs: registers::Registers::new(),
            mmap: _mmap,
            pc: 0x00,
            sp: 0x00,
            //bus: MemoryBus::new(boot_rom, game_rom),
            is_halted: false,
            interrupts_enabled: true,
            //instruction_builder: instruction_builder::InstructionBuilder::new(),
        }
    }

    pub fn execute_instruction(&mut self) {
        // let _nb = self.mmap.mem[self.pc as usize];
        // self.pc += 1;
        // println!("next_byte: 0x{:x?}", _nb);

        // let _nb = self.mmap.mem[self.pc as usize];
        // self.pc += 1;
        // println!("next_byte: 0x{:x?}", _nb);

        //instruction_builder::InstructionBuilder::test_func(self);
        //instruction_builder::InstructionBuilder::test_func(self.instruction_builder, self);
        //self.instruction_builder.test_func(self);
        // self.instruction_builder.test_func();
        // self.instruction_builder.test_func();
        // self.instruction_builder.test_func();
        //self.instruction_builder.test_func2();
    }

    pub fn next(&mut self) -> u8 {
        let _nb = self.mmap.mem[self.pc as usize];
        self.pc += 1;
        _nb
    }
}

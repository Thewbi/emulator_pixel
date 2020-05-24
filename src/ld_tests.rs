use super::cpu::CPU;
use super::instruction_targets::{from_load_word_target, LoadWordTarget};
use super::instructions::{Instruction, LD_N_NN};
use super::memory_map::MemoryMap;

#[test]
fn test_load_word() {
    // Arrange
    //
    let op_code = 0x01;

    let instruction = LD_N_NN::new(op_code, 12, from_load_word_target(op_code), 0x34, 0x56);

    assert_eq!(instruction.opcode, op_code);
    assert_eq!(instruction.cycles, 12);
    assert_eq!(instruction.target, LoadWordTarget::BC);
    assert_eq!(instruction.data1, 0x34);
    assert_eq!(instruction.data2, 0x56);

    let memory_map = MemoryMap::new();
    let mut cpu = CPU::new(memory_map);

    assert_eq!(cpu.regs.b, 0x00);
    assert_eq!(cpu.regs.c, 0x00);

    // Act
    //
    instruction.execute(&mut cpu);

    // Assert
    //
    assert_eq!(cpu.regs.b, 0x34);
    assert_eq!(cpu.regs.c, 0x56);
}

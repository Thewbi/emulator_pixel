use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::convert::TryFrom;
use std::option::Option;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IncDecTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
    BC,
    DE,
    HL,
    SP,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ADDHLTarget {
    BC,
    DE,
    HL,
    SP,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PrefixTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum JumpTest {
    NotZero,
    NotCarry,
    Zero,
    Carry,
    Always,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

#[allow(dead_code)]
#[derive(FromPrimitive, Copy, Clone, Debug, PartialEq)]
pub enum LoadWordTarget {
    BC = 0x01,
    DE = 0x11,
    HL = 0x21,
    SP = 0x31,
}
pub fn from_load_word_target(value: u8) -> LoadWordTarget {
    match FromPrimitive::from_u8(value) {
        std::option::Option::Some(LoadWordTarget::BC) => LoadWordTarget::BC,
        std::option::Option::Some(LoadWordTarget::DE) => LoadWordTarget::DE,
        std::option::Option::Some(LoadWordTarget::HL) => LoadWordTarget::HL,
        std::option::Option::Some(LoadWordTarget::SP) => LoadWordTarget::SP,
        None => panic!("Invalid value for Enumeration LoadWordTarget"),
    }
}
impl TryFrom<u8> for LoadWordTarget {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(LoadWordTarget::BC),
            0x11 => Ok(LoadWordTarget::DE),
            0x21 => Ok(LoadWordTarget::HL),
            0x31 => Ok(LoadWordTarget::SP),
            _ => Err("Unkown value for enum LoadWordTarget"),
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Indirect {
    BCIndirect,
    DEIndirect,
    HLIndirectMinus,
    HLIndirectPlus,
    WordIndirect,
    LastByteIndirect,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget),
    AFromIndirect(Indirect),
    IndirectFromA(Indirect),
    AFromByteAddress,
    ByteAddressFromA,
    SPFromHL,
    HLFromSPN,
    IndirectFromSP,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StackTarget {
    AF,
    BC,
    DE,
    HL,
}

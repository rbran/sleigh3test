use core::ops::RangeInclusive;
use std::io::Read;

use serde::Deserialize;
use sleigh4rust::Endian;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum Version {
    V4,
    V5,
    V6,
    V7,
    V8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Arm(u32),
    Thumb32(u16, u16),
    Thumb16(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
enum InstructionType {
    Arm,
    Thumb32,
    Thumb16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Test {
    pub versions: RangeInclusive<Version>,
    pub addr: u32,
    pub instruction: Instruction,
    pub result: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct TestSerialized {
    min_version: Version,
    max_version: Version,
    addr: u32,
    instruction_type: InstructionType,
    instruction_value: u32,
    result: String,
}

impl Instruction {
    fn new(inst_type: InstructionType, value: u32) -> Self {
        match inst_type {
            InstructionType::Arm => Self::Arm(value),
            InstructionType::Thumb32 => {
                Self::Thumb32((value >> 16) as u16, (value & 0xffff) as u16)
            }
            InstructionType::Thumb16 => {
                //TODO make this happen in the Deserialize and remove the unwrap
                Self::Thumb16(value.try_into().unwrap())
            }
        }
    }
}

impl From<TestSerialized> for Test {
    fn from(value: TestSerialized) -> Self {
        Self {
            versions: value.min_version..=value.max_version,
            addr: value.addr,
            instruction: Instruction::new(
                value.instruction_type,
                value.instruction_value,
            ),
            result: value.result,
        }
    }
}

impl Instruction {
    pub fn to_tokens(&self, endian: Endian) -> Vec<u8> {
        use Endian::*;
        match (self, endian) {
            (Self::Arm(x), Big) => x.to_be_bytes().to_vec(),
            (Self::Arm(x), Little) => x.to_le_bytes().to_vec(),
            (Self::Thumb32(x, y), Big) => {
                ((*x as u32) << 16 | *y as u32).to_be_bytes().to_vec()
            }
            (Self::Thumb32(x, y), Little) => {
                ((*y as u32) << 16 | *x as u32).to_le_bytes().to_vec()
            }
            (Self::Thumb16(x), Big) => x.to_be_bytes().to_vec(),
            (Self::Thumb16(x), Little) => x.to_le_bytes().to_vec(),
        }
    }
    pub fn thumb_mode(&self) -> bool {
        match self {
            Self::Arm(_) => false,
            Self::Thumb32(..) | Self::Thumb16(_) => true,
        }
    }
}

struct TestsFromFile<R>(csv::DeserializeRecordsIntoIter<R, TestSerialized>);
impl<R: Read> Iterator for TestsFromFile<R> {
    type Item = csv::Result<Test>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|test| test.map(Test::from))
    }
}

pub const BASIS_INSTRUCTION_FILE: &str = "../assets/arm/basic.csv";
pub fn tests_from_file<R: Read>(
    file: R,
) -> impl Iterator<Item = csv::Result<Test>> {
    TestsFromFile(
        csv::Reader::from_reader(file).into_deserialize::<TestSerialized>(),
    )
}

pub fn test_instruction(
    test: Test,
    version: Version,
    endian: Endian,
    parse_arm: fn(&[u8], u32) -> Option<(u32, String)>,
    parse_thumb: Option<fn(&[u8], u32) -> Option<(u32, String)>>,
) {
    //check the version
    if !test.versions.contains(&version) {
        return;
    }
    //only parse thumb if have it
    let token = test.instruction.to_tokens(endian);
    let unable_to_parse = || {
        panic!(
            "Unable to parse the {:x?} with expected output `{}`",
            &token, &test.result,
        );
    };
    let (next_addr, result) = match test.instruction {
        Instruction::Arm(_) => {
            parse_arm(&token, test.addr).unwrap_or_else(unable_to_parse)
        }
        Instruction::Thumb32(_, _) | Instruction::Thumb16(_) => {
            if let Some(parse_thumb) = parse_thumb {
                parse_thumb(&token, test.addr).unwrap_or_else(unable_to_parse)
            } else {
                return;
            }
        }
    };
    assert_eq!(next_addr, test.addr + token.len() as u32);
    assert_eq!(result, test.result);
}

pub fn tests_instruction_from_file(
    file: &str,
    version: Version,
    endian: Endian,
    parse_arm: fn(&[u8], u32) -> Option<(u32, String)>,
    parse_thumb: Option<fn(&[u8], u32) -> Option<(u32, String)>>,
) {
    let test_file = std::fs::File::open(file).unwrap();
    let tests = tests_from_file(test_file);
    for test in tests.map(Result::unwrap) {
        test_instruction(test, version, endian, parse_arm, parse_thumb);
    }
}

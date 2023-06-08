use core::ops::RangeInclusive;
use std::io::Read;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum Version {
    V4,
    V5,
    V6,
    V7,
    V8,
}

impl Version {
    pub fn number(&self) -> u8 {
        match self {
            Version::V4 => 4,
            Version::V5 => 5,
            Version::V6 => 6,
            Version::V7 => 7,
            Version::V8 => 8,
        }
    }
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
            instruction: Instruction::new(value.instruction_type, value.instruction_value),
            result: value.result,
        }
    }
}

impl Instruction {
    pub fn to_tokens(&self, big_endian: bool) -> Vec<u8> {
        match (self, big_endian) {
            (Self::Arm(x), true) => x.to_be_bytes().to_vec(),
            (Self::Arm(x), false) => x.to_le_bytes().to_vec(),
            (Self::Thumb32(x, y), true) => ((*x as u32) << 16 | *y as u32).to_be_bytes().to_vec(),
            (Self::Thumb32(x, y), false) => ((*y as u32) << 16 | *x as u32).to_le_bytes().to_vec(),
            (Self::Thumb16(x), true) => x.to_be_bytes().to_vec(),
            (Self::Thumb16(x), false) => x.to_le_bytes().to_vec(),
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
pub fn tests_from_file<R: Read>(file: R) -> impl Iterator<Item = csv::Result<Test>> {
    TestsFromFile(csv::Reader::from_reader(file).into_deserialize::<TestSerialized>())
}

//fn icicle(big_endian: bool, version: Version, thumb: bool) -> sleigh_runtime::SleighData {
//    let home = std::env::var("GHIDRA_SRC").unwrap();
//    let t = if version.number() > 5 { false } else { thumb };
//    let file_in = format!(
//        "{}/Ghidra/Processors/ARM/data/languages/ARM{}{}_{}e.slaspec",
//        home,
//        version.number(),
//        if t { "t" } else { "" },
//        if big_endian { 'b' } else { 'l' }
//    );
//    sleigh_compile::from_path(&file_in).unwrap()
//}

struct ParseStatic {
    parse_arm: fn(&[u8], u32) -> Option<(u32, String)>,
    parse_thumb: Option<fn(&[u8], u32) -> Option<(u32, String)>>,
}

trait Parse {
    fn arm(&mut self, token: &[u8], addr: u32) -> Option<(u32, String)>;
    fn have_thumb(&self) -> bool;
    fn thumb(&mut self, token: &[u8], addr: u32) -> Option<(u32, String)>;
}

impl Parse for ParseStatic {
    fn arm(&mut self, token: &[u8], addr: u32) -> Option<(u32, String)> {
        (self.parse_arm)(token, addr)
    }

    fn have_thumb(&self) -> bool {
        self.parse_thumb.is_some()
    }

    fn thumb(&mut self, token: &[u8], addr: u32) -> Option<(u32, String)> {
        self.parse_thumb.unwrap()(token, addr)
    }
}

//struct ParseIcicle {
//    icicle: sleigh_runtime::SleighData,
//    runtime: sleigh_runtime::Runtime,
//    tmode: Option<sleigh_runtime::ContextField>,
//}
//impl Parse for ParseIcicle {
//    fn arm(&mut self, token: &[u8], addr: u32) -> Option<(u32, String)> {
//        self.runtime.context = 0;
//        let instr = self.runtime.decode(&self.icicle, addr as u64, token)?;
//        let result = self.icicle.disasm(instr)?;
//        Some((self.runtime.get_instruction().inst_next as u32, result))
//    }
//
//    fn have_thumb(&self) -> bool {
//        self.tmode.is_some()
//    }
//
//    fn thumb(&mut self, token: &[u8], addr: u32) -> Option<(u32, String)> {
//        self.runtime.context = 0;
//        self.tmode.unwrap().field.set(&mut self.runtime.context, 1);
//        let instr = self.runtime.decode(&self.icicle, addr as u64, token)?;
//        let result = self.icicle.disasm(instr)?;
//        Some((self.runtime.get_instruction().inst_next as u32, result))
//    }
//}

fn test_instruction(test: &Test, version: Version, big_endian: bool, parse: &mut dyn Parse) {
    //check the version
    if !test.versions.contains(&version) {
        return;
    }
    //only parse thumb if have it
    let token = test.instruction.to_tokens(big_endian);
    let unable_to_parse = || {
        panic!(
            "Unable to parse the {:x?} with expected output `{}`",
            &token, &test.result,
        );
    };
    let (next_addr, result) = match test.instruction {
        Instruction::Arm(_) => parse.arm(&token, test.addr).unwrap_or_else(unable_to_parse),
        Instruction::Thumb32(_, _) | Instruction::Thumb16(_) => {
            if parse.have_thumb() {
                parse
                    .thumb(&token, test.addr)
                    .unwrap_or_else(unable_to_parse)
            } else {
                return;
            }
        }
    };
    assert_eq!(result, test.result, "at instruction {:?}", test.instruction);
    assert_eq!(
        next_addr,
        test.addr + token.len() as u32,
        "at instruction {:?}",
        test.instruction
    );
}

pub fn tests_instruction_from_file(
    file: &str,
    version: Version,
    big_endian: bool,
    parse_arm: fn(&[u8], u32) -> Option<(u32, String)>,
    parse_thumb: Option<fn(&[u8], u32) -> Option<(u32, String)>>,
) {
    let test_file = std::fs::File::open(file).unwrap();
    let tests = tests_from_file(test_file);
    let mut parse_static = ParseStatic {
        parse_arm,
        parse_thumb,
    };
    //let icicle = icicle(big_endian, version, parse_thumb.is_some());
    //let tmode = parse_thumb.map(|_| icicle.get_context_field("TMode").unwrap());
    //let mut parse_icicle = ParseIcicle {
    //    icicle,
    //    runtime: sleigh_runtime::Runtime::new(0),
    //    tmode,
    //};
    for test in tests.map(Result::unwrap) {
        test_instruction(&test, version, big_endian, &mut parse_static);
        // test with icicle too
        //test_instruction(&test, version, big_endian, &mut parse_icicle);
    }
}

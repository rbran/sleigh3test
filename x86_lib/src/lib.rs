use std::io::Read;

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub addr: u64,
    pub token: Vec<u8>,
    pub result: String,
}
#[derive(Debug, Clone, Deserialize)]
struct InstructionSerialized {
    pub addr: u64,
    pub token: String,
    pub result: String,
}
impl From<InstructionSerialized> for Instruction {
    fn from(value: InstructionSerialized) -> Self {
        //TODO make this happen inside the deserializer
        let token = value
            .token
            .as_bytes()
            .chunks(2)
            .map(String::from_utf8_lossy) //I'm lazy
            .map(|byte| u8::from_str_radix(&byte, 16).unwrap())
            .collect();
        Self {
            addr: value.addr,
            token,
            result: value.result,
        }
    }
}
struct TestsFromFile<R>(csv::DeserializeRecordsIntoIter<R, InstructionSerialized>);
impl<R: Read> Iterator for TestsFromFile<R> {
    type Item = csv::Result<Instruction>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|test| test.map(Instruction::from))
    }
}

pub const STRLEN_32_INSTRUCTION_FILE: &str = "../assets/x86/strlen_32.csv";
pub const STRLEN_64_INSTRUCTION_FILE: &str = "../assets/x86/strlen_64.csv";
pub fn tests_from_file<R: Read>(file: R) -> impl Iterator<Item = csv::Result<Instruction>> {
    TestsFromFile(csv::Reader::from_reader(file).into_deserialize::<InstructionSerialized>())
}

pub fn tests_instruction_from_file<A>(file: &str, parse: fn(&[u8], A) -> Option<(A, String)>)
where
    A: TryFrom<u64> + TryInto<u64> + core::fmt::Debug + std::ops::Add,
    u64: TryFrom<A>,
    <A as TryFrom<u64>>::Error: std::fmt::Debug,
    <u64 as TryFrom<A>>::Error: std::fmt::Debug,
{
    let test_file = std::fs::File::open(file).unwrap();
    let instructions = tests_from_file(test_file);
    for instruction in instructions.map(Result::unwrap) {
        let addr = A::try_from(instruction.addr).unwrap();
        let Some((next_addr, result)) = parse(&instruction.token, addr) else {
                panic!(
                    "Unable to parse the {:x?} with expected output `{}`",
                    &instruction.token,
                    &instruction.result,
                );
            };
        assert_eq!(result, instruction.result);
        assert_eq!(
            u64::try_from(next_addr).unwrap(),
            instruction.addr + instruction.token.len() as u64
        );
    }
}

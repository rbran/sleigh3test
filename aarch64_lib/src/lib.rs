use std::io::Read;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Instruction {
    pub addr: u64,
    token: u32,
    pub result: String,
}
impl Instruction {
    fn tokens(&self) -> [u8; 4] {
        self.token.to_le_bytes()
    }
}

struct TestsFromFile<R>(csv::DeserializeRecordsIntoIter<R, Instruction>);
impl<R: Read> Iterator for TestsFromFile<R> {
    type Item = csv::Result<Instruction>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|test| test.map(Instruction::from))
    }
}

pub const RANDOM_INSTRUCTION_FILE: &str = "../assets/aarch64/random.csv";
pub const RANDOM_BIG_INSTRUCTION_FILE: &str = "../assets/aarch64/random_big.csv";
pub fn tests_from_file<R: Read>(file: R) -> impl Iterator<Item = csv::Result<Instruction>> {
    TestsFromFile(csv::Reader::from_reader(file).into_deserialize::<Instruction>())
}

pub fn tests_instruction_from_file(file: &str, parse: fn(&[u8], u64) -> Option<(u64, String)>) {
    let test_file = std::fs::File::open(file).unwrap();
    let instructions = tests_from_file(test_file);
    for instruction in instructions.map(Result::unwrap) {
        let Some((next_addr, result)) = parse(&instruction.tokens(), instruction.addr) else {
                panic!(
                    "Unable to parse the {:x?} with expected output `{}`",
                    &instruction.token,
                    &instruction.result,
                );
            };
        assert_eq!(result, instruction.result);
        assert_eq!(next_addr, instruction.addr + 4);
    }
}

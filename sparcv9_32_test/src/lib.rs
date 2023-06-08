use std::io::Read;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Instruction {
    pub addr: u32,
    token: u32,
    pub result: String,
}
impl Instruction {
    fn to_tokens(&self) -> [u8; 4] {
        self.token.to_be_bytes()
    }
}

struct TestsFromFile<R>(csv::DeserializeRecordsIntoIter<R, Instruction>);
impl<R: Read> Iterator for TestsFromFile<R> {
    type Item = csv::Result<Instruction>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub const RANDOM_INSTRUCTION_FILE: &str = "../assets/sparcv9/random_32.csv";
pub fn tests_from_file<R: Read>(file: R) -> impl Iterator<Item = csv::Result<Instruction>> {
    TestsFromFile(csv::Reader::from_reader(file).into_deserialize::<Instruction>())
}

pub fn tests_instruction_from_file(file: &str, parse: fn(&[u8], u32) -> Option<(u32, String)>) {
    let test_file = std::fs::File::open(file).unwrap();
    let instructions = tests_from_file(test_file);
    for instruction in instructions.map(Result::unwrap) {
        let token = instruction.to_tokens();
        let Some((next_addr, result)) = parse(&token, instruction.addr) else {
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

#[cfg(test)]
mod test {
    use crate::*;
    use sparcv9_32::*;

    #[test]
    fn random() {
        tests_instruction_from_file(RANDOM_INSTRUCTION_FILE, parse_default)
    }
}

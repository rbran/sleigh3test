use std::io::Read;

use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum Token {
    Two(u16),
    Four(u32),
}
impl Token {
    fn to_tokens(&self) -> Vec<u8> {
        match self {
            Token::Two(x) => x.to_le_bytes().to_vec(),
            Token::Four(x) => x.to_le_bytes().to_vec(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub addr: u32,
    pub token: Token,
    pub result: String,
}

#[derive(Debug, Clone, Deserialize)]
pub enum TokenType {
    Two,
    Four,
}
#[derive(Debug, Clone, Deserialize)]
struct InstructionSerialized {
    pub addr: u32,
    pub token_value: u32,
    pub token_type: TokenType,
    pub result: String,
}
impl From<InstructionSerialized> for Instruction {
    fn from(value: InstructionSerialized) -> Self {
        let token = match value.token_type {
            TokenType::Two => Token::Two(value.token_value.try_into().unwrap()),
            TokenType::Four => Token::Four(value.token_value),
        };
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
pub fn tests_from_file<R: Read>(file: R) -> impl Iterator<Item = csv::Result<Instruction>> {
    TestsFromFile(csv::Reader::from_reader(file).into_deserialize::<InstructionSerialized>())
}

pub fn tests_instruction_from_file(file: &str, parse: fn(&[u8], u32) -> Option<(u32, String)>) {
    let test_file = std::fs::File::open(file).unwrap();
    let instructions = tests_from_file(test_file);
    for instruction in instructions.map(Result::unwrap) {
        let addr = instruction.addr;
        let token = instruction.token.to_tokens();
        let Some((next_addr, result)) = parse(&token, addr) else {
                panic!(
                    "Unable to parse the {:x?} with expected output `{}`",
                    &instruction.token,
                    &instruction.result,
                );
            };
        assert_eq!(result, instruction.result);
        assert_eq!(next_addr, instruction.addr + token.len() as u32);
    }
}
pub const RANDOM_INSTRUCTION_FILE: &str = "../assets/v850/random.csv";
pub const RANDOM_BIG_INSTRUCTION_FILE: &str = "../assets/v850/random_big.csv";

#[cfg(test)]
mod test {
    use v850::*;

    use crate::*;

    #[test]
    fn random() {
        tests_instruction_from_file(RANDOM_INSTRUCTION_FILE, parse_default)
    }
    #[test]
    #[ignore]
    fn random_big() {
        tests_instruction_from_file(RANDOM_BIG_INSTRUCTION_FILE, parse_default)
    }
}

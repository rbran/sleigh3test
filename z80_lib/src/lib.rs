use std::io::Read;

use serde::Deserialize;

pub const RANDOM_INSTRUCTION_FILE: &str = "../assets/z80/random.csv";
#[derive(Debug, Clone)]
pub enum Token {
    One(u8),
    Two(u16),
    Three(u32),
}
impl Token {
    fn to_tokens(&self) -> Vec<u8> {
        match self {
            Token::One(x) => vec![*x],
            Token::Two(x) => x.to_be_bytes().to_vec(),
            Token::Three(x) => x.to_be_bytes()[1..].to_vec(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub addr: u16,
    pub token: Token,
    pub result: String,
}

#[derive(Debug, Clone, Deserialize)]
pub enum TokenType {
    One,
    Two,
    Three,
}
#[derive(Debug, Clone, Deserialize)]
struct InstructionSerialized {
    pub addr: u16,
    pub token_value: u32,
    pub token_type: TokenType,
    pub result: String,
}
impl From<InstructionSerialized> for Instruction {
    fn from(value: InstructionSerialized) -> Self {
        let token = match value.token_type {
            TokenType::One => Token::One(value.token_value.try_into().unwrap()),
            TokenType::Two => Token::Two(value.token_value.try_into().unwrap()),
            TokenType::Three => Token::Three(value.token_value),
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

pub fn tests_instruction_from_file(file: &str, parse: fn(&[u8], u16) -> Option<(u16, String)>) {
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
        assert_eq!(next_addr, instruction.addr + token.len() as u16);
    }
}

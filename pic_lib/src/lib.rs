use std::io::Read;

use serde::Deserialize;
use sleigh4rust::Endian;

#[derive(Debug, Clone, Deserialize)]
pub struct Instruction {
    pub addr: u16,
    token: u16,
    pub result: String,
}
impl Instruction {
    fn to_tokens(&self, endian: Endian) -> [u8; 2] {
        match endian {
            Endian::Big => self.token.to_be_bytes(),
            Endian::Little => self.token.to_le_bytes(),
        }
    }
}

struct TestsFromFile<R>(csv::DeserializeRecordsIntoIter<R, Instruction>);
impl<R: Read> Iterator for TestsFromFile<R> {
    type Item = csv::Result<Instruction>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub const GOTO_INSTRUCTION_FILE: &str = "../assets/pic/goto.csv";
pub fn tests_from_file<R: Read>(
    file: R,
) -> impl Iterator<Item = csv::Result<Instruction>> {
    TestsFromFile(
        csv::Reader::from_reader(file).into_deserialize::<Instruction>(),
    )
}

pub fn tests_instruction_from_file(
    file: &str,
    endian: Endian,
    parse: fn(&[u8], u16) -> Option<(u16, String)>,
) {
    let test_file = std::fs::File::open(file).unwrap();
    let instructions = tests_from_file(test_file);
    for instruction in instructions.map(Result::unwrap) {
        let token = instruction.to_tokens(endian);
        let Some((next_addr, result)) = parse(&token, instruction.addr) else {
                panic!(
                    "Unable to parse the {:x?} with expected output `{}`",
                    &instruction.token,
                    &instruction.result,
                );
            };
        assert_eq!(result, instruction.result);
        assert_eq!(next_addr, instruction.addr + 1);
    }
}

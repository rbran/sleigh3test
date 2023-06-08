use std::io::Read;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Instruction {
    pub addr: u32,
    token: u16,
    pub result: String,
}
impl Instruction {
    fn to_tokens(&self, big_endian: bool) -> [u8; 2] {
        if big_endian {
            self.token.to_be_bytes()
        } else {
            self.token.to_le_bytes()
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

pub const MOV_INSTRUCTION_FILE: &str = "../assets/superh4/mov.csv";
pub fn tests_from_file<R: Read>(file: R) -> impl Iterator<Item = csv::Result<Instruction>> {
    TestsFromFile(csv::Reader::from_reader(file).into_deserialize::<Instruction>())
}

pub fn tests_instruction_from_file(
    file: &str,
    big_endian: bool,
    parse: fn(&[u8], u32) -> Option<(u32, String)>,
) {
    let test_file = std::fs::File::open(file).unwrap();
    let instructions = tests_from_file(test_file);
    for instruction in instructions.map(Result::unwrap) {
        let token = instruction.to_tokens(big_endian);
        let addr = instruction.addr;
        let Some((next_addr, result)) = parse(&token, addr) else {
                panic!(
                    "Unable to parse the {:x?} with expected output `{}`",
                    &token,
                    &addr,
                );
            };
        assert_eq!(result, instruction.result);
        assert_eq!(next_addr, addr + token.len() as u32);
    }
}

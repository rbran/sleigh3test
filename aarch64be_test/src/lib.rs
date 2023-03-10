#[cfg(test)]
mod test {
    use aarch64_lib::*;
    use aarch64be::*;

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

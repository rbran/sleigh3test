#[cfg(test)]
mod test {
    use z180::*;
    use z80_lib::*;

    #[test]
    fn random() {
        tests_instruction_from_file(RANDOM_INSTRUCTION_FILE, parse_default)
    }
}

#[cfg(test)]
mod test {
    use arm7_le::*;
    use arm_lib::*;

    #[test]
    fn basic_instructions() {
        tests_instruction_from_file(
            BASIS_INSTRUCTION_FILE,
            Version::V7,
            false,
            parse_arm,
            Some(parse_thumb),
        );
    }
}

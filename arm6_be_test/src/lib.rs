#[cfg(test)]
mod test {
    use arm6_be::*;
    use arm_lib::*;

    #[test]
    fn basic_instructions() {
        tests_instruction_from_file(
            BASIS_INSTRUCTION_FILE,
            Version::V6,
            true,
            parse_arm,
            Some(parse_thumb),
        );
    }
}

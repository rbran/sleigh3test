#[cfg(test)]
mod test {
    use arm8_be::*;
    use arm_lib::*;

    #[test]
    fn basic_instructions() {
        tests_instruction_from_file(
            BASIS_INSTRUCTION_FILE,
            Version::V8,
            true,
            parse_arm,
            Some(parse_thumb),
        );
    }
}

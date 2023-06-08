#[cfg(test)]
mod test {
    use arm5t_le::*;
    use arm_lib::*;

    #[test]
    fn basic_instructions() {
        tests_instruction_from_file(
            BASIS_INSTRUCTION_FILE,
            Version::V5,
            false,
            parse_arm,
            Some(parse_thumb),
        );
    }
}

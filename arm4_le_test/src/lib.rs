#[cfg(test)]
mod test {
    use arm4_le::*;
    use arm_lib::*;

    #[test]
    fn basic_instructions() {
        tests_instruction_from_file(BASIS_INSTRUCTION_FILE, Version::V4, false, parse_arm, None);
    }
}

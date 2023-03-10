#[cfg(test)]
mod test {
    use arm7_be::*;
    use arm_lib::*;
    use sleigh4rust::Endian;

    #[test]
    fn basic_instructions() {
        tests_instruction_from_file(
            BASIS_INSTRUCTION_FILE,
            Version::V7,
            Endian::Big,
            parse_arm,
            Some(parse_thumb),
        );
    }
}

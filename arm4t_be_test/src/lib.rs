#[cfg(test)]
mod test {
    use arm4t_be::*;
    use arm_lib::*;
    use sleigh4rust::Endian;

    #[test]
    fn basic_instructions() {
        tests_instruction_from_file(
            BASIS_INSTRUCTION_FILE,
            Version::V4,
            Endian::Big,
            parse_arm,
            Some(parse_thumb),
        );
    }
}

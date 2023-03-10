#[cfg(test)]
mod test {
    use sleigh4rust::Endian;
    use superh4_le::*;
    use superh4_lib::*;

    #[test]
    fn random() {
        tests_instruction_from_file(
            MOV_INSTRUCTION_FILE,
            Endian::Little,
            parse_default,
        )
    }
}

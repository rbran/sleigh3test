#[cfg(test)]
mod test {
    use x86_64::*;
    use x86_lib::*;
    #[test]
    fn strlen_32() {
        tests_instruction_from_file(
            STRLEN_32_INSTRUCTION_FILE,
            parse_64bits_emu32,
        )
    }
    #[test]
    fn strlen_64() {
        tests_instruction_from_file(STRLEN_64_INSTRUCTION_FILE, parse_64bits)
    }
}

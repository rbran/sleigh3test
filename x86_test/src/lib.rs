#[cfg(test)]
mod test {
    use x86::*;
    use x86_lib::*;

    #[test]
    fn strlen() {
        tests_instruction_from_file::<u32>(
            STRLEN_32_INSTRUCTION_FILE,
            parse_32bits,
        )
    }
}

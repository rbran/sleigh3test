#[cfg(test)]
mod test {
    use pic16c5x::*;
    use pic_lib::*;

    #[test]
    #[ignore]
    fn goto() {
        tests_instruction_from_file(GOTO_INSTRUCTION_FILE, false, parse_default)
    }
}

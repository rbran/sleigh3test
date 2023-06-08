#[cfg(test)]
mod test {
    use pic12c5xx::*;
    use pic_lib::*;

    #[test]
    #[ignore]
    fn goto() {
        tests_instruction_from_file(GOTO_INSTRUCTION_FILE, false, parse_default)
    }
}

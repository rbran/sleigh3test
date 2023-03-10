#[cfg(test)]
mod test {
    use pic12c5xx::*;
    use pic_lib::*;
    use sleigh4rust::Endian;

    #[test]
    #[ignore]
    fn goto() {
        tests_instruction_from_file(
            GOTO_INSTRUCTION_FILE,
            Endian::Little,
            parse_default,
        )
    }
}

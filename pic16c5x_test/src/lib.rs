#[cfg(test)]
mod test {
    use pic16c5x::*;
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

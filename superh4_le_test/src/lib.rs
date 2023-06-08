#[cfg(test)]
mod test {

    use superh4_le::*;
    use superh4_lib::*;

    #[test]
    fn random() {
        tests_instruction_from_file(MOV_INSTRUCTION_FILE, false, parse_default)
    }
}

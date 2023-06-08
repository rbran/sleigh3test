#[cfg(test)]
mod test {

    use superh4_be::*;
    use superh4_lib::*;

    #[test]
    fn mov() {
        tests_instruction_from_file(MOV_INSTRUCTION_FILE, true, parse_default)
    }
}

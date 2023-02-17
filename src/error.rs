#[derive(Debug)]
pub enum IOError {
    InvalidDigraphHeader,
}

#[cfg(test)]
mod testing {
    use super::IOError;

    #[test]
    fn test_error() {
        let err = IOError::InvalidDigraphHeader;
        println!("{:?}", err);
    }
}

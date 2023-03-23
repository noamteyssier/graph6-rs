#[derive(Debug, PartialEq, Eq)]
pub enum IOError {
    InvalidDigraphHeader,
    InvalidSizeChar,
    GraphTooLarge,
    InvalidAdjacencyMatrix,
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

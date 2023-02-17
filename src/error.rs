#[derive(Debug)]
pub enum Graph6Error {
    InvalidDigraphHeader,
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_error() {
        let err = Graph6Error::InvalidDigraphHeader;
        println!("{:?}", err);
    }
}

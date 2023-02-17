use super::IOError;

/// Returns the size of the graph
pub fn get_size(bytes: &[u8], pos: usize) -> Result<usize, IOError> {
    let size = bytes[pos];
    if size == 126 {
        Err(IOError::GraphTooLarge)
    } else if size < 63 {
        Err(IOError::InvalidSizeChar)
    } else {
        Ok((size - 63) as usize)
    }
}

#[cfg(test)]
mod testing {
    use super::get_size;

    #[test]
    fn test_pos_0() {
        let bytes = b"AG";
        let size = get_size(bytes, 0).unwrap();
        assert_eq!(size, 2);
    }

    #[test]
    fn test_pos_1() {
        let bytes = b"&AG";
        let size = get_size(bytes, 1).unwrap();
        assert_eq!(size, 2);
    }

    #[test]
    fn test_oversize() {
        let bytes = b"~AG";
        let size = get_size(bytes, 0).unwrap_err();
        assert_eq!(size, super::IOError::GraphTooLarge);
    }

    #[test]
    fn test_invalid_size_char() {
        let bytes = b">AG";
        let size = get_size(bytes, 0).unwrap_err();
        assert_eq!(size, super::IOError::InvalidSizeChar);
    }
}

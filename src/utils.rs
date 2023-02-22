use super::IOError;

/// Iterates through the bytes of a graph and fills a bitvector representing
/// the adjacency matrix of the graph
pub fn fill_bitvector(bytes: &[u8], n: usize, offset: usize) -> Vec<usize> {
    let mut bit_vec = Vec::with_capacity(n * n);
    for byte in bytes.iter().skip(offset).take(n) {
        let byte = byte - 63;
        for shift in (0..6).rev() {
            if (byte & 1 << shift) > 0 {
                bit_vec.push(1);
            } else {
                bit_vec.push(0);
            }
        }
    }
    bit_vec
}

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

/// Returns the upper triangle of a bitvector
pub fn upper_triangle(bit_vec: &[usize], n: usize) -> Vec<usize> {
    let mut tri = Vec::with_capacity(n * (n - 1) / 2);
    for i in 1..n {
        for j in 0..i {
            let idx = i * n + j;
            tri.push(bit_vec[idx])
        }
    }
    tri
}

#[cfg(test)]
mod testing {
    use super::get_size;

    #[test]
    fn test_size_pos_0() {
        let bytes = b"AG";
        let size = get_size(bytes, 0).unwrap();
        assert_eq!(size, 2);
    }

    #[test]
    fn test_size_pos_1() {
        let bytes = b"&AG";
        let size = get_size(bytes, 1).unwrap();
        assert_eq!(size, 2);
    }

    #[test]
    fn test_size_oversize() {
        let bytes = b"~AG";
        let size = get_size(bytes, 0).unwrap_err();
        assert_eq!(size, super::IOError::GraphTooLarge);
    }

    #[test]
    fn test_size_invalid_size_char() {
        let bytes = b">AG";
        let size = get_size(bytes, 0).unwrap_err();
        assert_eq!(size, super::IOError::InvalidSizeChar);
    }

    #[test]
    fn test_bitvector() {
        let bytes = b"GG";
        let bit_vec = super::fill_bitvector(bytes, 2, 0);
        assert_eq!(bit_vec, vec![0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0]);
    }

    #[test]
    fn test_bitvector_offset() {
        let bytes = b"AG";
        let bit_vec = super::fill_bitvector(bytes, 2, 1);
        assert_eq!(bit_vec, vec![0, 0, 1, 0, 0, 0]);
    }

    #[test]
    fn test_upper_triangle_n2() {
        let bit_vec = vec![0, 1, 1, 0];
        let tri = super::upper_triangle(&bit_vec, 2);
        assert_eq!(tri, vec![1]);
    }

    #[test]
    fn test_upper_triangle_n3() {
        let bit_vec = vec![0, 1, 1, 1, 0, 0, 1, 0, 0];
        let tri = super::upper_triangle(&bit_vec, 3);
        assert_eq!(tri, vec![1, 1, 0]);
    }
}

use crate::{GraphConversion, utils::upper_triangle};

/// Trait to write graphs into graph 6 formatted strings
pub trait WriteGraph: GraphConversion {
    fn write_graph(&self) -> String {
        write_graph6(self.bit_vec().to_vec(), self.size(), self.is_directed())
    }
}

fn write_header(repr: &mut String, is_directed: bool) {
    if is_directed {
        repr.push('&');
    }
}

fn write_size(repr: &mut String, size: usize) {
    let size_char = char::from_u32(size as u32 + 63).unwrap();
    repr.push(size_char);
}

fn pad_bitvector(bit_vec: &mut Vec<usize>) {
    if bit_vec.len() % 6 != 0 {
        (0..6 - (bit_vec.len() % 6)).for_each(|_| bit_vec.push(0));
    }
}

fn parse_bitvector(bit_vec: &[usize], repr: &mut String) {
    for chunk in bit_vec.chunks(6) {
        let mut sum = 0;
        for (i, bit) in chunk.iter().rev().enumerate() {
            sum += bit * 2usize.pow(i as u32);
        }
        let char = char::from_u32(sum as u32 + 63).unwrap();
        repr.push(char);
    }
}

pub fn write_graph6(bit_vec: Vec<usize>, n: usize, is_directed: bool) -> String {
    let mut repr = String::new();
    let mut bit_vec = if is_directed {
        bit_vec
    } else {
        upper_triangle(&bit_vec, n)
    };
    write_header(&mut repr, is_directed);
    write_size(&mut repr, n);
    pad_bitvector(&mut bit_vec);
    parse_bitvector(&bit_vec, &mut repr);
    repr
}

#[cfg(test)]
mod testing {

    #[test]
    fn test_write_undirected_n2() {
        let bit_vec = vec![0, 1, 1, 0];
        let repr = super::write_graph6(bit_vec, 2, false);
        assert_eq!(repr, "A_");
    }

    #[test]
    fn test_write_directed_n2_mirror() {
        let bit_vec = vec![0, 1, 1, 0];
        let repr = super::write_graph6(bit_vec, 2, true);
        assert_eq!(repr, "&AW");
    }


    #[test]
    fn test_write_directed_n2_unmirrored() {
        let bit_vec = vec![0, 0, 1, 0];
        let repr = super::write_graph6(bit_vec, 2, true);
        assert_eq!(repr, "&AG");
    }

}

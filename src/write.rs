use crate::GraphConversion;

/// Trait to write graphs into graph 6 formatted strings
pub trait WriteGraph: GraphConversion {

    fn owned_bit_vec(&self) -> Vec<usize>;

    fn write_header(&self, repr: &mut String) {
        if self.is_directed() {
            repr.push('&');
        }
    }
    
    fn write_size(&self, repr: &mut String) {
        let size_char = char::from_u32(self.size() as u32 + 63).unwrap();
        repr.push(size_char);
    }

    fn pad_bitvector(&self, bit_vec: &mut Vec<usize>) {
        if bit_vec.len() % 6 != 0 {
            (0..6 - (bit_vec.len() % 6)).for_each(|_| bit_vec.push(0));
        }
    }

    fn parse_bitvector(&self, bit_vec: &mut Vec<usize>, repr: &mut String) {
        for chunk in bit_vec.chunks(6) {
            let mut sum = 0;
            for (i, bit) in chunk.iter().rev().enumerate() {
                sum += bit * 2usize.pow(i as u32);
            }
            let char = char::from_u32(sum as u32 + 63).unwrap();
            repr.push(char);
        }
    }

    fn write_graph(&self) -> String {
        let mut repr = String::new();
        let mut bv = self.owned_bit_vec();
        self.write_header(&mut repr);
        self.write_size(&mut repr);
        self.pad_bitvector(&mut bv);
        self.parse_bitvector(&mut bv, &mut repr);
        repr
    }
}

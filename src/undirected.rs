use crate::utils::get_size;
use super::IOError;

/// Creates an undirected graph from a graph6 representation
pub struct Graph {
    bit_vec: Vec<usize>,
    n: usize,
}
impl Graph {

    pub fn from_g6(repr: &str) -> Result<Self, IOError> {
        let bytes = repr.as_bytes();
        let n = get_size(bytes, 0)?;
        let bit_vec = Self::build_bitvector(bytes, n);
        Ok(Self {bit_vec, n })
    }

    fn build_bitvector(bytes: &[u8], n: usize) -> Vec<usize> {
        let bv_len = n * (n-1) / 2;

        let mut bit_vec = Vec::with_capacity(bv_len);
        for byte in bytes.iter().skip(1).take(n) {
            let byte = byte - 63;
            for shift in (0..6).rev() {
                if (byte & 1 << shift) > 0 {
                    bit_vec.push(1);
                } else {
                    bit_vec.push(0);
                }
            }
        }
        Self::adjust_bitvector_len(&mut bit_vec, bv_len);
        Self::fill_from_triangle(&bit_vec, n)
    }

    fn adjust_bitvector_len(bit_vec: &mut Vec<usize>, bv_len: usize) {
        let adj_bv_len = bit_vec.len() - (bit_vec.len() - bv_len);
        bit_vec.truncate(adj_bv_len);
    }

    fn fill_from_triangle(tri: &[usize], n: usize) -> Vec<usize> {
        let mut bit_vec = vec![0; n * n];
        let mut tri_iter = tri.iter();
        for i in 1..n {
            for j in 0..i {
                let idx = i * n + j;
                let jdx = j * n + i;
                let val = *tri_iter.next().unwrap();
                bit_vec[idx] = val;
                bit_vec[jdx] = val;
            }
        }
        bit_vec
    }

    /// Returns the bitvector representing the flattened adjacency matrix of the graph
    pub fn bit_vec(&self) -> &[usize] {
        &self.bit_vec
    }

    /// Returns the size of the graph (number of vertices)
    pub fn size(&self) -> usize {
        self.n
    }

}

#[cfg(test)]
mod testing {
    use super::Graph;

    #[test]
    fn test_graph_n2() {
        let graph = Graph::from_g6("A_").unwrap();
        assert_eq!(graph.size(), 2);
        assert_eq!(graph.bit_vec(), &[0, 1, 1, 0]);
    }

    #[test]
    fn test_graph_n2_empty() {
        let graph = Graph::from_g6("A?").unwrap();
        assert_eq!(graph.size(), 2);
        assert_eq!(graph.bit_vec(), &[0, 0, 0, 0]);
    }

    #[test]
    fn test_graph_n3() {
        let graph = Graph::from_g6("Bw").unwrap();
        assert_eq!(graph.size(), 3);
        assert_eq!(graph.bit_vec(), &[0, 1, 1, 1, 0, 1, 1, 1, 0]);
    }

    #[test]
    fn test_graph_n4() {
        let graph = Graph::from_g6("C~").unwrap();
        assert_eq!(graph.size(), 4);
        assert_eq!(graph.bit_vec(), &[0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0]);
    }
}

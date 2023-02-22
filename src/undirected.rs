use super::{GraphConversion, IOError};
use crate::{
    utils::{fill_bitvector, get_size, upper_triangle},
    WriteGraph,
};

/// Creates an undirected graph from a graph6 representation
#[derive(Debug)]
pub struct Graph {
    pub bit_vec: Vec<usize>,
    pub n: usize,
}
impl Graph {
    /// Creates a new undirected graph from a graph6 representation
    ///
    /// # Arguments
    /// * `repr` - A graph6 representation of the graph
    ///
    /// # Example
    /// ```
    /// use graph6_rs::Graph;
    /// let graph = Graph::from_g6("A_").unwrap();
    /// assert_eq!(graph.n, 2);
    /// assert_eq!(graph.bit_vec, &[0, 1, 1, 0]);
    /// ```
    pub fn from_g6(repr: &str) -> Result<Self, IOError> {
        let bytes = repr.as_bytes();
        let n = get_size(bytes, 0)?;
        let bit_vec = Self::build_bitvector(bytes, n);
        Ok(Self { bit_vec, n })
    }

    /// Builds the bitvector from the graph6 representation
    fn build_bitvector(bytes: &[u8], n: usize) -> Vec<usize> {
        let bv_len = n * (n - 1) / 2;
        let mut bit_vec = fill_bitvector(bytes, n, 1);
        Self::adjust_bitvector_len(&mut bit_vec, bv_len);
        Self::fill_from_triangle(&bit_vec, n)
    }

    /// Adjusts the length of the bitvector to the correct length
    fn adjust_bitvector_len(bit_vec: &mut Vec<usize>, bv_len: usize) {
        let adj_bv_len = bit_vec.len() - (bit_vec.len() - bv_len);
        bit_vec.truncate(adj_bv_len);
    }

    /// Fills the adjacency bitvector from an upper triangle
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

    /// Returns the upper triangle of the adjacency matrix
    fn upper_triangle(&self) -> Vec<usize> {
        upper_triangle(&self.bit_vec, self.n)
    }
}
impl GraphConversion for Graph {
    /// Returns the bitvector representation of the graph
    fn bit_vec(&self) -> &[usize] {
        &self.bit_vec
    }

    /// Returns the number of vertices in the graph
    fn size(&self) -> usize {
        self.n
    }

    /// Returns true if the graph is directed
    fn is_directed(&self) -> bool {
        false
    }
}
impl WriteGraph for Graph {
    fn owned_bit_vec(&self) -> Vec<usize> {
        self.upper_triangle()
    }
}

#[cfg(test)]
mod testing {
    use super::{Graph, GraphConversion, WriteGraph};

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
        assert_eq!(
            graph.bit_vec(),
            &[0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0]
        );
    }

    #[test]
    fn test_to_adjacency() {
        let graph = Graph::from_g6("A_").unwrap();
        let adj = graph.to_adjmat();
        assert_eq!(adj, "0 1\n1 0\n");
    }

    #[test]
    fn test_to_dot() {
        let graph = Graph::from_g6("A_").unwrap();
        let dot = graph.to_dot(None);
        assert_eq!(dot, "graph {\n0 -- 1;\n}");
    }

    #[test]
    fn test_to_dot_with_label() {
        let graph = Graph::from_g6("A_").unwrap();
        let dot = graph.to_dot(Some(1));
        assert_eq!(dot, "graph graph_1 {\n0 -- 1;\n}");
    }

    #[test]
    fn test_to_net() {
        let repr = r"A_";
        let graph = Graph::from_g6(repr).unwrap();
        let net = graph.to_net();
        assert_eq!(net, "*Vertices 2\n1 \"0\"\n2 \"1\"\n*Arcs\n1 2\n2 1\n");
    }

    #[test]
    fn test_write_n2() {
        let repr = r"A_";
        let graph = Graph::from_g6(repr).unwrap();
        let g6 = graph.write_graph();
        assert_eq!(g6, repr);
    }

    #[test]
    fn test_write_n3() {
        let repr = r"Bw";
        let graph = Graph::from_g6(repr).unwrap();
        let g6 = graph.write_graph();
        assert_eq!(g6, repr);
    }

    #[test]
    fn test_write_n4() {
        let repr = r"C~";
        let graph = Graph::from_g6(repr).unwrap();
        let g6 = graph.write_graph();
        assert_eq!(g6, repr);
    }
}

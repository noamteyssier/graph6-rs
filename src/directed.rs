use super::{GraphConversion, IOError};
use crate::{
    utils::{fill_bitvector, get_size},
    WriteGraph,
};

/// Creates a directed graph from a graph6 representation
#[derive(Debug)]
pub struct DiGraph {
    pub bit_vec: Vec<usize>,
    pub n: usize,
}
impl DiGraph {
    /// Creates a new DiGraph from a graph6 representation string
    ///
    /// # Arguments
    /// * `repr` - A graph6 representation string
    ///
    /// # Errors
    /// Returns an error if the graph6 representation is invalid (i.e. missing digraph header '&')
    ///
    /// # Examples
    /// ```
    /// use graph6_rs::DiGraph;
    /// let graph = DiGraph::from_d6("&AG").unwrap();
    /// assert_eq!(graph.n, 2);
    /// assert_eq!(graph.bit_vec, &[0, 0, 1, 0]);
    /// ```
    pub fn from_d6(repr: &str) -> Result<Self, IOError> {
        let bytes = repr.as_bytes();
        Self::valid_digraph(bytes)?;
        let n = get_size(bytes, 1)?;
        let bit_vec = Self::build_bitvector(bytes, n);
        Ok(Self { bit_vec, n })
    }

    /// Validates graph6 directed representation
    fn valid_digraph(repr: &[u8]) -> Result<bool, IOError> {
        if repr[0] == b'&' {
            Ok(true)
        } else {
            Err(IOError::InvalidDigraphHeader)
        }
    }

    /// Iteratores through the bytes and builds a bitvector
    /// representing the adjaceny matrix of the graph
    fn build_bitvector(bytes: &[u8], n: usize) -> Vec<usize> {
        let bv_len = n * n;
        let mut bit_vec = fill_bitvector(bytes, n, 2);
        Self::adjust_bitvector_len(&mut bit_vec, bv_len);
        bit_vec
    }

    /// Adjusts the bitvector length to the expected size of the digraph (n*n)
    fn adjust_bitvector_len(bit_vec: &mut Vec<usize>, bv_len: usize) {
        let adj_bv_len = bit_vec.len() - (bit_vec.len() - (bv_len));
        bit_vec.truncate(adj_bv_len);
    }
}

impl GraphConversion for DiGraph {
    fn bit_vec(&self) -> &[usize] {
        &self.bit_vec
    }

    fn size(&self) -> usize {
        self.n
    }

    fn is_directed(&self) -> bool {
        true
    }
}

impl WriteGraph for DiGraph {
    fn owned_bit_vec(&self) -> Vec<usize> {
        self.bit_vec.clone()
    }
}

#[cfg(test)]
mod testing {
    use crate::WriteGraph;

    use super::GraphConversion;

    #[test]
    fn test_header() {
        let repr = b"&AG";
        assert!(super::DiGraph::valid_digraph(repr).is_ok());
    }

    #[test]
    fn test_invalid_header() {
        let repr = b"AG";
        assert!(super::DiGraph::valid_digraph(repr).is_err());
    }

    #[test]
    /// Adjacency matrix:
    /// 0 1
    /// 1 0
    fn test_bitvector_n2() {
        let repr = "&AG";
        let graph = super::DiGraph::from_d6(repr).unwrap();
        assert_eq!(graph.size(), 2);
        assert_eq!(graph.bit_vec(), vec![0, 0, 1, 0]);
    }

    #[test]
    /// Adjacency matrix:
    /// 0 1 1
    /// 1 0 1
    /// 1 1 0
    fn test_bitvector_n3() {
        let repr = r"&B\o";
        let graph = super::DiGraph::from_d6(repr).unwrap();
        assert_eq!(graph.size(), 3);
        assert_eq!(graph.bit_vec(), vec![0, 1, 1, 1, 0, 1, 1, 1, 0]);
    }

    #[test]
    /// Adjacency matrix:
    /// 0 1 1 1
    /// 1 0 1 1
    /// 1 1 0 1
    /// 1 1 1 0
    fn test_bitvector_n4() {
        let repr = r"&C]|w";
        let graph = super::DiGraph::from_d6(repr).unwrap();
        assert_eq!(graph.size(), 4);
        assert_eq!(
            graph.bit_vec(),
            vec![0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0]
        );
    }

    #[test]
    fn test_init_invalid_n2() {
        let repr = "AG";
        let graph = super::DiGraph::from_d6(repr);
        assert!(graph.is_err());
    }

    #[test]
    fn test_to_adjacency() {
        let repr = r"&C]|w";
        let graph = super::DiGraph::from_d6(repr).unwrap();
        let adj = graph.to_adjmat();
        assert_eq!(adj, "0 1 1 1\n1 0 1 1\n1 1 0 1\n1 1 1 0\n");
    }

    #[test]
    fn test_to_dot() {
        let repr = r"&AG";
        let graph = super::DiGraph::from_d6(repr).unwrap();
        let dot = graph.to_dot(None);
        assert_eq!(dot, "digraph {\n1 -> 0;\n}");
    }

    #[test]
    fn test_to_dot_with_id() {
        let repr = r"&AG";
        let graph = super::DiGraph::from_d6(repr).unwrap();
        let dot = graph.to_dot(Some(1));
        assert_eq!(dot, "digraph graph_1 {\n1 -> 0;\n}");
    }

    #[test]
    fn test_to_net() {
        let repr = r"&AG";
        let graph = super::DiGraph::from_d6(repr).unwrap();
        let net = graph.to_net();
        assert_eq!(net, "*Vertices 2\n1 \"0\"\n2 \"1\"\n*Arcs\n2 1\n");
    }

    #[test]
    fn test_write_n2() {
        let repr = r"&AG";
        let graph = super::DiGraph::from_d6(repr).unwrap();
        let graph6 = graph.write_graph();
        assert_eq!(graph6, repr);
    }

    #[test]
    fn test_write_n3() {
        let repr = r"&B\o";
        let graph = super::DiGraph::from_d6(repr).unwrap();
        let graph6 = graph.write_graph();
        assert_eq!(graph6, repr);
    }

    #[test]
    fn test_write_n4() {
        let repr = r"&C]|w";
        let graph = super::DiGraph::from_d6(repr).unwrap();
        let graph6 = graph.write_graph();
        assert_eq!(graph6, repr);
    }
}

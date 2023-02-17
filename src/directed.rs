use super::IOError;

/// Creates a directed graph from a graph6 representation
pub struct DiGraph {
    bit_vec: Vec<usize>,
    n: usize,
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
    /// assert_eq!(graph.size(), 2);
    /// assert_eq!(graph.bit_vec(), &[0, 0, 1, 0]);
    /// ```
    pub fn from_d6(repr: &str) -> Result<Self, IOError> {
        let bytes = repr.as_bytes();

        if !Self::valid_digraph(bytes) {
            return Err(IOError::InvalidDigraphHeader)
        }
        let n = Self::get_size(bytes);
        let bit_vec = Self::build_bitvector(bytes, n);
        Ok(Self{ bit_vec, n })
    }

    /// Validates graph6 directed representation
    fn valid_digraph(repr: &[u8]) -> bool {
        repr[0] == b'&'
    }

    /// Returns the size of the graph
    fn get_size(repr: &[u8]) -> usize {
        (repr[1] - 63) as usize
    }

    /// Iteratores through the bytes and builds a bitvector
    /// representing the adjaceny matrix of the graph
    fn build_bitvector(bytes: &[u8], n: usize) -> Vec<usize> {
        let bv_len = n * n;

        let mut bit_vec = Vec::with_capacity(bv_len);
        for byte in bytes.iter().skip(2).take(n) {
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
        bit_vec
    }

    /// Adjusts the bitvector length to the expected size of the digraph (n*n)
    fn adjust_bitvector_len(bit_vec: &mut Vec<usize>, bv_len: usize) {
        let adj_bv_len = bit_vec.len() - (bit_vec.len() - (bv_len));
        bit_vec.resize(adj_bv_len, 0);
    }

    /// Writes graph as adjacency matrix
    pub fn to_adjacency(&self) -> String {
        let mut adj = String::new();
        for i in 0..self.n {
            for j in 0..self.n {
                adj.push_str(&format!("{} ", self.bit_vec[i * self.n + j]));
            }
            adj.push_str("\n");
        }
        adj
    }

    /// Writes graph as a DOT file
    pub fn to_dot(&self, id: Option<usize>) -> String {
        let mut dot = String::new();
        if let Some(id) = id {
            dot.push_str(&format!(r"digraph graph_{} {{", id));
        } else {
            dot.push_str("digraph {");
        }
        for i in 0..self.n {
            for j in 0..self.n {
                if self.bit_vec[i * self.n + j] == 1 {
                    dot.push_str(&format!("\n{} -> {};", i, j));
                }
            }
        }
        dot.push_str("\n}");
        dot
    }

    /// Writes graph as Pajeck .NET file
    pub fn to_net(&self) -> String {
        let mut net = String::new();
        net.push_str(&format!("*Vertices {}\n", self.n));
        for i in 0..self.n {
            net.push_str(&format!("{} \"{}\"\n", i + 1, i));
        }
        net.push_str("*Arcs\n");
        for i in 0..self.n {
            for j in 0..self.n {
                if self.bit_vec[i * self.n + j] == 1 {
                    net.push_str(&format!("{} {}\n", i + 1, j + 1));
                }
            }
        }
        net

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

    #[test]
    fn test_header() {
        let repr = b"&AG";
        assert!(super::DiGraph::valid_digraph(repr));
    }

    #[test]
    fn test_invalid_header() {
        let repr = b"AG";
        assert!(!super::DiGraph::valid_digraph(repr));
    }

    #[test]
    fn test_size() {
        assert_eq!(super::DiGraph::get_size(b"&AG"), 2);
        assert_eq!(super::DiGraph::get_size(b"&BG"), 3);
        assert_eq!(super::DiGraph::get_size(b"&CG"), 4);
        assert_eq!(super::DiGraph::get_size(b"&DG"), 5);
    }

    #[test]
    /// Adjacency matrix:
    /// 0 1
    /// 1 0
    fn test_bitvector_n2() {
        let repr = b"&AG";
        let n = super::DiGraph::get_size(repr);
        let bit_vec = super::DiGraph::build_bitvector(repr, n);
        assert_eq!(bit_vec, vec![0, 0, 1, 0]);
    }

    #[test]
    /// Adjacency matrix:
    /// 0 1 1
    /// 1 0 1
    /// 1 1 0
    fn test_bitvector_n3() {
        let repr = br"&B\o";
        let n = super::DiGraph::get_size(repr);
        let bit_vec = super::DiGraph::build_bitvector(repr, n);
        assert_eq!(bit_vec, vec![0, 1, 1, 1, 0, 1, 1, 1, 0]);
    }

    #[test]
    /// Adjacency matrix:
    /// 0 1 1 1
    /// 1 0 1 1
    /// 1 1 0 1
    /// 1 1 1 0
    fn test_bitvector_n4() {
        let repr = br"&C]|w";
        let n = super::DiGraph::get_size(repr);
        let bit_vec = super::DiGraph::build_bitvector(repr, n);
        assert_eq!(bit_vec, vec![0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0]);
    }

    #[test]
    fn test_init_n2() {
        let repr = "&AG";
        let graph = super::DiGraph::from_d6(repr).unwrap();
        assert_eq!(graph.size(), 2);
        assert_eq!(graph.bit_vec(), vec![0, 0, 1, 0]);
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
        let adj = graph.to_adjacency();
        assert_eq!(adj, "0 1 1 1 \n1 0 1 1 \n1 1 0 1 \n1 1 1 0 \n");
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
}

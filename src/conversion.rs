
/// Conversion trait for graphs into various text graph formats
pub trait GraphConversion {

    /// Returns the bitvector representation of the graph
    fn bit_vec(&self) -> &[usize];

    /// Returns the number of vertices in the graph
    fn size(&self) -> usize;

    /// Returns true if the graph is directed
    fn is_directed(&self) -> bool;

    /// Returns the graph in the DOT format
    fn to_dot(&self, id: Option<usize>) -> String {
        let n = self.size();
        let bit_vec = self.bit_vec();

        let mut dot = String::new();

        // include graph type
        if self.is_directed() {
            dot.push_str("digraph ");
        } else {
            dot.push_str("graph ");
        }

        // include graph id
        if let Some(id) = id {
            dot.push_str(&format!("graph_{} {{", id));
        } else {
            dot.push_str("{");
        }

        // include edges
        if self.is_directed() {
            self.to_directed_dot(&mut dot, bit_vec, n);
        } else {
            self.to_undirected_dot(&mut dot, bit_vec, n);
        }

        // close graph
        dot.push_str("\n}");

        dot
    }

    fn to_undirected_dot(&self, dot: &mut String, bit_vec: &[usize], n: usize) {
        for i in 0..n {
            for j in i..n {
                if bit_vec[i * n + j] == 1 {
                    dot.push_str(&format!("\n{} -- {};", i, j));
                }
            }
        }
    }

    fn to_directed_dot(&self, dot: &mut String, bit_vec: &[usize], n: usize) {
        for i in 0..n {
            for j in 0..n {
                if bit_vec[i * n + j] == 1 {
                    dot.push_str(&format!("\n{} -> {};", i, j));
                }
            }
        }
    }

    /// Returns the graph as an adjacency matrix
    fn to_adjmat(&self) -> String {
        let n = self.size();
        let bit_vec = self.bit_vec();

        let mut adj = String::new();
        for i in 0..n {
            for j in 0..n {
                adj.push_str(&format!("{}", bit_vec[i * n + j]));
                if j < n - 1 {
                    adj.push_str(" ");
                }
            }
            adj.push_str("\n");
        }
        adj
    }

    /// Returns the graph in the Pajek NET format
    fn to_net(&self) -> String {
        let n = self.size();
        let bit_vec = self.bit_vec();

        let mut net = String::new();
        net.push_str(&format!("*Vertices {}\n", n));
        for i in 0..n {
            net.push_str(&format!("{} \"{}\"\n", i + 1, i));
        }
        net.push_str("*Arcs\n");
        for i in 0..n {
            for j in 0..n {
                if bit_vec[i * n + j] == 1 {
                    net.push_str(&format!("{} {}\n", i + 1, j + 1));
                }
            }
        }
        net
    }
}

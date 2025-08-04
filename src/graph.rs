use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node(pub usize);

#[derive(Debug, Clone)]
pub struct Edge {
    pub u: Node,
    pub v: Node,
    pub weight: i64,
}

#[derive(Debug)]
pub struct Graph {
    pub edges: Vec<Edge>,
    nodes: HashSet<Node>

}

impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: Vec::new(),
            nodes: HashSet::new()

        }
    }

    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn add_edge(&mut self, u: usize, v: usize, weight: i64) {
        self.edges.push(Edge {
            u: Node(u),
            v: Node(v),
            weight,
        });
        self.nodes.insert(Node(u));
        self.nodes.insert(Node(v));
    }

    pub fn kruskal_mst(&mut self) -> (Vec<Edge>, i64) {
        self.sort_edges();
        self.kruskal_from_sorted_edges()
    }

    pub fn sort_edges(&mut self)  {
        self.edges.sort_by_key(|e| e.weight);

    }

    pub fn kruskal_from_sorted_edges(&self) -> (Vec<Edge>, i64) {
        let mut uf = UnionFind::new(self.num_nodes());
        let mut mst_edges = Vec::with_capacity(self.num_nodes());
        let mut total_weight = 0;

        for edge in self.edges.iter() {
            if uf.union(edge.u.0, edge.v.0) {
                total_weight += edge.weight;
                mst_edges.push(edge.clone());
                if mst_edges.len() == self.num_nodes() - 1 {
                    break
                }
            }
        }

        (mst_edges, total_weight)
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    #[inline]
    fn find(&mut self, x: usize) -> usize {
        let mut y = x;
        unsafe {
            let mut p = *self.parent.get_unchecked(y);
            while y != p {
                let grandparent =  *self.parent.get_unchecked(p);
                *self.parent.get_unchecked_mut(y) = grandparent;
                y = p;
                p = grandparent;
            }
        }
        y
    }

    #[inline]
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                self.parent[root_x] = root_y;
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x;
            }
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kruskal_mst() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1, 10);
        graph.add_edge(0, 2, 6);
        graph.add_edge(0, 3, 5);
        graph.add_edge(1, 3, 15);
        graph.add_edge(2, 3, 4);

        let (mst_edges, total_weight) = graph.kruskal_mst();
        
        assert_eq!(mst_edges.len(), 3);
        assert_eq!(total_weight, 19);
    }

    #[test]
    fn test_disconnected_graph() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1, 1);
        graph.add_edge(2, 3, 1);

        let (mst_edges, total_weight) = graph.kruskal_mst();
        
        assert_eq!(mst_edges.len(), 2);
        assert_eq!(total_weight, 2);
    }
}
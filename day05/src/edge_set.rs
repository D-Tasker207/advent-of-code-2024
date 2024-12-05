use std::collections::HashMap;
use std::collections::HashSet;

pub struct EdgeSet {
    pub edges: HashMap<i32, HashSet<i32>>
}

impl EdgeSet {
    pub fn new() -> Self {
        return EdgeSet {
            edges: HashMap::new()
        }
    }

    pub fn new_from_edges(edges: Vec<Vec<i32>>) -> Self {
        let mut edge_set = EdgeSet::new();
        for edge in edges {
            edge_set.add_edge(edge[0], edge[1]);
        }
        return edge_set;
    }

    pub fn has_edge(&self, src: i32, dest: i32) -> bool {
        match self.edges.get(&src) {
            Some(edges) => {
                if edges.contains(&dest) {
                    return true;
                }
            },
            None => return false,
        }
        return false;
    }

    pub fn add_edge(&mut self, src: i32, dest: i32) {
        match self.edges.get(&src) {
            Some(edges) => {
                if !edges.contains(&dest) {
                    self.edges.get_mut(&src).unwrap().insert(dest);
                }
            }
            None => {
                let mut new_set = HashSet::new();
                new_set.insert(dest);
                self.edges.insert(src, new_set);
            }
        }
    }
}
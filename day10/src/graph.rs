use std::collections::{HashSet, HashMap};

pub struct NodeData {
    pub value: i32,
    pub edges: HashSet<(i32, i32)>,
}

pub struct Graph {
    nodes_by_position: HashMap<(i32, i32), NodeData>,
    position_by_value: HashMap<i32, HashSet<(i32, i32)>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes_by_position: HashMap::new(),
            position_by_value: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, position: (i32, i32), value: i32, edges: HashSet<(i32, i32)>) {
        let node_data = NodeData { value, edges };

        self.nodes_by_position.insert(position, node_data);

        self.position_by_value
            .entry(value)
            .or_insert(HashSet::new())
            .insert(position);
    }

    pub fn get_node_by_posn(&self, posn: (i32, i32)) -> Option<&NodeData> {
        self.nodes_by_position.get(&posn)
    }

    pub fn get_posn_by_value(&self, value: i32) -> Option<&HashSet<(i32, i32)>> {
        self.position_by_value.get(&value)
    }
}
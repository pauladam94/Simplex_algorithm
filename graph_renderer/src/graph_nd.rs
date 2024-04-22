use crate::graph_2d::Graph2D;
use crate::node_nd::{pos_to_node, NodeND};

#[derive(Debug)]
pub struct GraphND {
    pub nodes: Vec<NodeND>,
    pub adjacencies: Vec<Vec<usize>>,
}

impl GraphND {
    pub fn new(positions: Vec<Vec<f32>>, adjacencies: Vec<Vec<usize>>) -> Self {
        let mut max_id = 0;
        let mut nodes: Vec<NodeND> = Vec::new();
        for i in 0..positions.len() {
            nodes.push(pos_to_node(max_id, positions[i].clone()));
            max_id += 1
        }
        let result = Self {
            nodes,
            adjacencies,
        };
        if result.adjacencies.len() != positions.len() {
            println!("{result}");
            panic!("The number of nodes and the number of adjacency lists must be the same");
        }
        result
    }


    pub fn add_edge(&mut self, id1: usize, id2: usize) {
        if !self.adjacencies[id1].contains(&id2)
            && !self.adjacencies[id2].contains(&id1)
            && id1 != id2
        {
            self.adjacencies[id1].push(id2);
            self.adjacencies[id2].push(id1);
        }
    }

    pub fn add_every_edge(&mut self) {
        for i in 0..self.nodes.len() {
            for j in 0..self.nodes.len() {
                if i != j {
                    self.add_edge(i, j);
                }
            }
        }
    }

    pub fn projection2d(&mut self) -> Graph2D {
        let mut nodes = Vec::new();
        for node in self.nodes.iter() {
            nodes.push(node.projection_2d());
        }
        Graph2D::new(nodes, self.adjacencies.clone())
    }

    pub fn rotate_z(&mut self, angle: f32) {
        // rotate every node

        for i in 0..self.nodes.len() {
            self.nodes[i].rotate_z(angle);
        }
    }
}

impl std::fmt::Display for GraphND {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nodes : ")?;
        for node in self.nodes.iter() {
            writeln!(f, "| {node} |")?;
        }
        writeln!(f)?;

        writeln!(f, "Adjacencies :")?;

        for (i, adj) in self.adjacencies.iter().enumerate() {
            write!(f, "\t {i} -> ")?;
            for j in adj.iter() {
                write!(f, "| {j} |")?;
            }
            writeln!(f)?;
        }
        // writeln!(f, "Max id : {}", self.node[0].len())?;
        Ok(())
    }
}

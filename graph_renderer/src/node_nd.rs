use crate::node_2d::Node2D;

#[derive(Debug)]
pub struct NodeND {
    pub id: usize,
    pub center: Vec<f32>
}

impl NodeND {
    pub fn new(id :usize, center: Vec<f32>) -> Self {
        Self {
            id,
            center,
        }
    }

    pub fn projection_2d(&self) -> Node2D {
        Node2D::new(self.id, egui::Pos2::new(self.center[0], self.center[1]))
    }

    pub fn rotate_z(&mut self, angle: f32) {
        let x = self.center[0];
        let y = self.center[1];
        self.center[0] = x * angle.cos() - y * angle.sin();
        self.center[1] = x * angle.sin() + y * angle.cos();
    }
}

impl std::fmt::Display for NodeND {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {},  pos: ", self.id)?;
        for i in 0..self.center.len() {
            write!(f, "{:.1} ", self.center[i])?;
        }
        Ok(())
    }
}

pub fn pos_to_node(id: usize, pos: Vec<f32>) -> NodeND {
    NodeND {
        id,
        center : pos
    }
}


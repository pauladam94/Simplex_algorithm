use crate::node_2d::Node2D;

#[derive(Debug, Clone)]
pub struct Graph2D {
    pub nodes: Vec<Node2D>,
    pub adjacencies: Vec<Vec<usize>>,
}

impl Graph2D {
    pub fn new(nodes: Vec<Node2D>, adjacencies: Vec<Vec<usize>>) -> Self {
        Self { nodes, adjacencies }
    }

    // pub fn add_node(&mut self, pos: egui::Pos2) {
    //    self.nodes.push(node::pos2_to_node(self.max_id, pos));
    //    self.max_id += 1;
    //    self.adjacencies.push(Vec::new());
    // }

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

    pub fn draw(
        &mut self,
        ui: &mut egui::Ui,
        stroke_line: egui::Stroke,
        stroke_node: egui::Stroke,
    ) {
        for (i, node) in self.nodes.iter().enumerate() {
            for j in self.adjacencies[i].iter() {
                ui.painter()
                    .line_segment([node.center, self.nodes[*j].center], stroke_line);
            }
        }
        for node in self.nodes.iter() {
            node.clone().draw(ui, stroke_node)
        }
    }

    pub fn center_window(&mut self, ui: &egui::Ui) {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;
        for node in self.nodes.iter() {
            if node.center.x < min_x {
                min_x = node.center.x;
            }
            if node.center.x > max_x {
                max_x = node.center.x;
            }
            if node.center.y < min_y {
                min_y = node.center.y;
            }
            if node.center.y > max_y {
                max_y = node.center.y;
            }
        }
        let center = egui::Pos2::new((min_x + max_x) / 2.0, (min_y + max_y) / 2.0);
        let window_center = ui.available_rect_before_wrap().center();
        let offset = window_center - center;
        for node in self.nodes.iter_mut() {
            node.center += offset;
        }
    }
}

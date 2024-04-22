use crate::graph_2d::Graph2D;
use crate::graph_nd::GraphND;
pub struct Graph {
    graph_2d : Graph2D,
    graph_nd : GraphND,
}

impl Graph {
    pub fn new(positions: Vec<Vec<f32>>, adjacencies: Vec<Vec<usize>>) -> Self {
        let mut graph_nd = GraphND::new(positions, adjacencies);
        Self {
            graph_2d : graph_nd.projection2d(),
            graph_nd,
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, stroke_line: egui::Stroke, stroke_node: egui::Stroke) {
        self.graph_2d.draw(ui, stroke_line, stroke_node);
    }
    pub fn rotate(&mut self, angle: f32) {
        self.graph_nd.rotate_z(angle);
        self.graph_2d = self.graph_nd.projection2d();
    }

    pub fn center_window(&mut self, ui: &egui::Ui) {
        self.graph_2d.center_window(ui);
        
    }

    
    pub fn add_every_edge(&mut self) {
        self.graph_2d.add_every_edge();
        self.graph_nd.add_every_edge();
    }
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.graph_nd)
    }
}

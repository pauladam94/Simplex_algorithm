
#[derive(Debug, Clone)]
pub struct Node2D {
    pub id: usize,
    pub center: egui::Pos2,
    pub radius: f32,
}

impl Node2D {
    pub fn new(id: usize, center: egui::Pos2) -> Self {
        Self {
            id,
            center,
            radius: 10.0,
        }
    }

    pub fn draw(&self, ui: &mut egui::Ui, stroke: egui::Stroke) {
        ui.painter().circle_stroke(self.center, self.radius, stroke);
        ui.painter()
            .circle_filled(self.center, self.radius, egui::Color32::WHITE);
        
    }
}




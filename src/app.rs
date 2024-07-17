use crate::constraint::Constraints;
use crate::linear_function::LinearFunction;
use crate::{Simplex, SimplexError};
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::{Body, Button, Heading, Monospace, Small};
use egui::{Color32, Style};
use graph_renderer::Graph;

pub struct App {
    maximize: bool,

    function_input: String,

    constraints_input: String,

    simplex: Option<Result<Simplex, SimplexError>>,

    graph: Option<Graph>,
    show_graph: bool,
    graph_stroke_line: egui::Stroke,
    graph_stroke_node: egui::Stroke,
}

impl Default for App {
    fn default() -> Self {
        Self {
            maximize: true,
            function_input: String::from("x + 6y + 13z"),
            constraints_input: String::from(
                "\
        x <= 200\n\
        y <= 300\n\
        x + y + z <= 400\n\
        y + 3z <= 600\n
                    ",
            ),
            simplex: None,

            graph: None,
            show_graph: true,
            graph_stroke_line: egui::Stroke::new(0.5, egui::Color32::GREEN),
            graph_stroke_node: egui::Stroke::new(0.5, egui::Color32::WHITE),
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            maximize,

            function_input,

            constraints_input,

            simplex,
            graph,
            show_graph,
            graph_stroke_line,
            graph_stroke_node,
        } = self;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            if ui.button("Rotate graph").clicked() || ui.input(|i| i.key_pressed(egui::Key::R)) {
                if let Some(real_graph) = graph {
                    real_graph.rotate(10.0);
                }
            }

            ui.checkbox(show_graph, "Show graph");

            if let Some(real_graph) = graph {
                ui.label(format!("{real_graph}"));
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if *show_graph {
                if let Some(real_graph) = graph {
                    real_graph.draw(ui, *graph_stroke_line, *graph_stroke_node)
                }
            }
        });

        // Change font sizes
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Body, FontId::new(24.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(14.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
        .into();
        ctx.set_style(style);

        egui::Area::new(egui::Id::new("Linear Program"))
            .default_pos(egui::pos2(32f32, 512f32))
            .show(ctx, |ui| {
                egui::Frame::window(&Style::default())
                    .fill(Color32::BLACK)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.heading("Linear Program");
                            ui.horizontal(|ui| {
                                egui::ComboBox::from_label("")
                                    .selected_text(
                                        (if *maximize { "MAX" } else { "MIN" }).to_string(),
                                    )
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(maximize, true, "MAX");
                                        ui.selectable_value(maximize, false, "MIN");
                                    });
                                ui.text_edit_singleline(function_input);
                            });
                            ui.text_edit_multiline(constraints_input);

                            if ui.add(egui::Button::new("COMPILE")).clicked() {
                                // Parse constraints
                                let constraints = Constraints::compile(constraints_input).unwrap();
                                // Parse linear function
                                let function =
                                    function_input.parse().unwrap_or(LinearFunction::zero());

                                // Create simplex
                                *simplex = Some(constraints.maximize(&if *maximize {
                                    function
                                } else {
                                    -function
                                }));

                                // create graph
                                // if let Some(Ok(simplex)) = simplex {
                                //     let every_points = simplex.every_points();
                                //     *graph = Some(Graph::new(
                                //         every_points.clone(),
                                //         // no adjacencies for anyone
                                //         vec![vec![]; every_points.len()],
                                //     ));
                                //     if let Some(real_graph) = graph {
                                //         real_graph.add_every_edge();
                                //         real_graph.center_window(ui)
                                //     }
                                // }
                                // add every node

                                // Show graph
                                // *show_simplex = true;

                                // Draw simplex
                                // self.simplex.draw();
                            }
                        });
                    })
            });

        egui::Area::new(egui::Id::new("State"))
            .default_pos(egui::pos2(512f32, 512f32))
            .show(ctx, |ui| {
                egui::Frame::window(&Style::default())
                    .fill(Color32::BLACK)
                    .show(ui, |ui| {
                        ui.vertical(|ui| match &self.simplex {
                            Some(Ok(simplex)) => {
                                ui.heading("Values");
                                let values = simplex.current_values();
                                ui.label(values.iter().fold(String::new(), |acc, (v, c)| {
                                    format!("{acc}{v} = {c}\n")
                                }));

                                ui.heading("State");
                                let current_state = simplex.current_state();
                                ui.colored_label(
                                    Color32::RED,
                                    format!("max {}", current_state.linear_function),
                                );
                                ui.label(current_state.constraints.to_string());
                            }
                            Some(Err(SimplexError::Unbounded)) => {
                                ui.colored_label(Color32::RED, "This program is unbounded");
                            }
                            None => {
                                ui.label("Press RUN to start the algorithm");
                            }
                            _ => {
                                ui.label("How did we get there ?");
                            }
                        });

                        ui.horizontal(|ui| {
                            // Previous button
                            if ui.add(egui::Button::new("PREVIOUS")).clicked() {
                                if let Some(Ok(simplex)) = &mut self.simplex {
                                    simplex.previous_step();
                                }
                            }
                            // Next button
                            if ui.add(egui::Button::new("NEXT")).clicked() {
                                if let Some(Ok(simplex)) = &mut self.simplex {
                                    match simplex.next_step(true) {
                                        Ok(()) => {}
                                        Err(_) => {}
                                    };
                                }
                            }
                        })
                    })
            });
    }
}

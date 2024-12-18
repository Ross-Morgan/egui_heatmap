#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use egui::ViewportBuilder;
use egui_heatmap::{
    Color, ColorWithThickness, MultiBitmapWidget, MultiBitmapWidgetSettings, MultiMapPosition,
    ShowState,
};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1000.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Multi-Map: Many data",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    bitmap: MultiBitmapWidget<usize>,
    state: ShowState<usize>,
}

impl Default for MyApp {
    fn default() -> Self {
        let settings = MultiBitmapWidgetSettings {
            start_size: None,
            id: "test".to_owned(),
            boundary_between_data: ColorWithThickness {
                color: Color::DARK_GRAY,
                thickness: 10,
            },
            colorbar: Some((
                egui_heatmap::colors::Gradient::with_options(
                    &egui_heatmap::colors::ColorGradientOptions::StartCenterEnd {
                        start: egui::Color32::RED,
                        center: egui::Color32::DARK_GREEN,
                        end: egui::Color32::BLUE,
                        steps: 64,
                    },
                ),
                80,
                (-3.1, 12.412564),
            )),
            background: Color::BLACK,
            boundary_unselected: ColorWithThickness {
                color: Color::GRAY,
                thickness: 7,
            },
            boundary_selected: Color::WHITE,
            boundary_factor_min: 3,
        };
        let bitmap = MultiBitmapWidget::with_settings(
            vec![
                egui_heatmap::Data::<Color>::example(
                    10,
                    20,
                    egui_heatmap::CoordinatePoint { x: 2, y: 8 },
                ),
                egui_heatmap::Data::<Color>::example(
                    10,
                    20,
                    egui_heatmap::CoordinatePoint { x: 10, y: 8 },
                ),
                egui_heatmap::Data::<Color>::example(
                    20,
                    40,
                    egui_heatmap::CoordinatePoint { x: 0, y: 0 },
                ),
            ]
            .into_iter()
            .enumerate()
            .collect(),
            settings,
        );
        Self {
            state: bitmap.default_state_english(),
            bitmap,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::left_to_right(egui::Align::BOTTOM).with_cross_justify(true),
                |ui| {
                    ui.vertical(|ui| {
                        ui.label("bla");
                    });
                    ui.with_layout(
                        egui::Layout::bottom_up(egui::Align::LEFT).with_cross_justify(true),
                        |ui| {
                            let problem = self.state.render_problem().map_or_else(
                                || "no problems".to_string(),
                                |e| format!("Problem: {e:?}"),
                            );
                            ui.label(problem);
                            // mouse over text
                            let text = match self.state.hover() {
                                MultiMapPosition::NotHovering => "-----".to_owned(),
                                MultiMapPosition::NoData(
                                    key,
                                    egui_heatmap::CoordinatePoint { x, y },
                                ) => format!("Plot #{key}: no data at {x}|{y}"),
                                MultiMapPosition::Pixel(
                                    key,
                                    egui_heatmap::CoordinatePoint { x, y },
                                ) => {
                                    format!("Plot #{key}: {x}|{y}")
                                }
                                MultiMapPosition::Colorbar(value) => {
                                    format!("Colorbar: {value:.5E}")
                                }
                            };
                            ui.label(text);
                            ui.label(
                                "Selected: ".to_owned()
                                    + &self
                                        .state
                                        .selected()
                                        .iter()
                                        .map(|egui_heatmap::CoordinatePoint { x, y }| {
                                            format!("({x}|{y})")
                                        })
                                        .collect::<Vec<_>>()
                                        .join(", "),
                            );
                            ui.label(
                                "Events: ".to_owned()
                                    + &self
                                        .state
                                        .events()
                                        .into_iter()
                                        .map(|e| format!("{e:?}"))
                                        .collect::<Vec<_>>()
                                        .join(", "),
                            );

                            self.bitmap.ui(ui, &mut self.state);
                        },
                    );
                },
            );
        });
    }
}

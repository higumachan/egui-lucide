use egui::color_picker::{color_picker_color32, Alpha};
use egui::{Color32, Slider, Widget};
use egui_extras::install_image_loaders;
use egui_lucide::Icon;

pub struct App {
    icon_size: u32,
    color: Color32,
    stroke_width: u32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            icon_size: 24,
            color: Color32::from_rgb(0, 0, 0),
            stroke_width: 1,
        }
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let mut app = App::default();
    eframe::run_simple_native("stack check", options, move |ctx, _frame| {
        install_image_loaders(ctx);

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("icon size:");
                    Slider::new(&mut app.icon_size, 0..=100)
                        .text("Icon size")
                        .ui(ui);
                });
                ui.horizontal(|ui| {
                    ui.label("color:");
                    color_picker_color32(ui, &mut app.color, Alpha::Opaque);
                });
                ui.horizontal(|ui| {
                    ui.label("stroke width:");
                    Slider::new(&mut app.stroke_width, 0..=10)
                        .text("Stroke width")
                        .ui(ui);
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            Icon::activity()
                .size(app.icon_size)
                .color(&app.color)
                .stroke_width(app.stroke_width)
                .ui(ui);
            Icon::lock()
                .size(app.icon_size)
                .color(&app.color)
                .stroke_width(app.stroke_width)
                .ui(ui);
        });
    })
}

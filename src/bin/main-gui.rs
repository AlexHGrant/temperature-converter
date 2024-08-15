#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Temperature Converter",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    temperature: f32,
    scale: temperatureconverter::Scale,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            temperature: 32.0,
            scale: temperatureconverter::Scale::Fahrenheit,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Temperature Converter");
            ui.horizontal(|ui| {
                ui.label("Select Scale");

                egui::ComboBox::from_id_source("scale-selector")
                .selected_text(format!("{0:?}", self.scale))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.scale, temperatureconverter::Scale::Celsius, "Celsius");
                    ui.selectable_value(&mut self.scale, temperatureconverter::Scale::Kelvin, "Kelvin");
                    ui.selectable_value(&mut self.scale, temperatureconverter::Scale::Fahrenheit, "Fahrenheit");
                });
            });
            ui.horizontal(|ui| {
                ui.label("Input Temperature");
                ui.add(egui::DragValue::new(&mut self.temperature)
                    .speed(0.01)
                    .range(-9999.99..=9999.99)
                );
            });
            let scale_in: temperatureconverter::Scale = self.scale;
            ui.label(get_temps(self.temperature, self.scale));
        });
    }
}

fn get_temps(temp: f32, scale: temperatureconverter::Scale) -> String{
    let t = temperatureconverter::calculate((scale, temp));
    return format!("{:?}: {}\n{:?}: {}\n{:?}: {}", 
        t.0.0, t.0.1, t.1.0, t.1.1, t.2.0, t.2.1);
}
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
use std::sync::mpsc::{Receiver, Sender};

use std::time::Duration;

use eframe::egui;

use egui::*;

use temperatureconverter::*;

use tokio::runtime::Runtime;

fn main() -> eframe::Result {

    let rt = Runtime::new().expect("Unable to create Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    // Execute the runtime in its own thread.
    // The future doesn't have to do anything. In this example, it just sleeps forever.
    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([320.0, 240.0]),
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
    tx: Sender<String>,
    rx: Receiver<String>,
    temperature: f32,
    scale: Scale,
    zip: String,
    zipout: String,
    history: String
}

impl Default for MyApp {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            tx,
            rx,
            temperature: 32.0,
            scale: Scale::Fahrenheit,
            zip: "20500".to_string(),
            zipout: "Press Go!".to_string(),
            history: "".to_string()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {

        if let Ok(zipout) = self.rx.try_recv() {
            self.zipout = zipout;
        }

        CentralPanel::default().show(ctx, |ui| {
            CollapsingHeader::new("Temperature Converter")
            .default_open(true)
            .show(ui, |ui| { 
                ui.horizontal(|ui| {
                    ui.label("Select Scale");
    
                    ComboBox::from_id_source("scale-selector")
                    .selected_text(format!("{0:?}", self.scale))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.scale, temperatureconverter::Scale::Celsius, "Celsius");
                        ui.selectable_value(&mut self.scale, temperatureconverter::Scale::Kelvin, "Kelvin");
                        ui.selectable_value(&mut self.scale, temperatureconverter::Scale::Fahrenheit, "Fahrenheit");
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("Input Temperature");
                    ui.add(DragValue::new(&mut self.temperature)
                        .speed(0.01)
                        .range(-9999.99..=9999.99)
                    );
                });
                ui.label(RichText::new(conv_temps(self.temperature, self.scale)).color(Color32::from_rgb(110, 255, 110)));
            });

            CollapsingHeader::new("ZIP Lookup")
            .default_open(false)
            .show(ui, |ui| { 
                ui.horizontal(|ui| {
                    ui.label("Input Zip");
                    ui.text_edit_singleline(&mut self.zip);
                });
                if ui.button("Go!").clicked() {
                    get_temps_from_zip(&self.zip, ctx.clone(), self.tx.clone());
                }
                ui.label(RichText::new(&self.zipout).color(Color32::from_rgb(110, 255, 110)));
            });                

            CollapsingHeader::new("Use History")
            .default_open(false)
            .show(ui, |ui| { 
                if ui.button("Update").clicked() {
                    self.history = match read_from_file() {
                        Ok(t) => t,
                        Err(e) => e.to_string()
                    }
                }
                ScrollArea::vertical().show(ui, |ui| {
                    ui.label(&self.history);
                });
            });     
        });
    }
}

fn conv_temps(temp: f32, scale: Scale) -> String{
    let t = calculate((scale, temp));
    return format!("{:?}: {}\n{:?}: {}\n{:?}: {}", 
        t.0.0, t.0.1, t.1.0, t.1.1, t.2.0, t.2.1);
}

fn get_temps_from_zip(input: &String, ctx: Context, tx: Sender<String>) {
    let zip = input.to_string();
    tokio::spawn(async move {
        let mut output:String = "".to_string();
        let get = get_current_temp(zip.to_string()).await;
        match get {
            Ok(t) => output = {
                let r = calculate((Scale::Celsius, t.2));
                format!(
                "Temperature in {}, {}\n{:?}: {}\n{:?}: {}\n{:?}: {}", 
                t.0, t.1, r.0.0, r.0.1, r.1.0, r.1.1, r.2.0, r.2.1)
            },
            Err(e) => output = e.to_string()
        }
        let _ = tx.send(output);
        ctx.request_repaint();
    });
}
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
    let _enter = rt.enter();

    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });

    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Temperature Converter",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Debug, PartialEq)]
enum Page {
    Temp,
    Zip,
    Hist
}

struct MyApp {
    tx: Sender<String>,
    rx: Receiver<String>,
    temperature: f32,
    scale: Scale,
    zip: String,
    zipout: String,
    history: String,
    page: Page
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
            history: "".to_string(),
            page: Page::Temp
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {

        if let Ok(zipout) = self.rx.try_recv() {
            self.zipout = zipout;
        }

        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.page, Page::Temp, "Converter");
                ui.selectable_value(&mut self.page, Page::Zip, "Zip Code Lookup");
                ui.selectable_value(&mut self.page, Page::Hist, "History");

            });

            ui.separator();
            if (self.page == Page::Temp) {
                ui.heading("Converter");
                ui.horizontal(|ui| {
                    ui.label("Select Scale");
    
                    ComboBox::from_id_source("scale-selector")
                    .selected_text(format!("{0:?}", self.scale))
                    .show_ui(ui, |ui| {
                        if ui.selectable_value(&mut self.scale, Scale::Celsius, "Celsius").clicked() {
                            let _ = write_to_file( &format!(
                                "Temperature converted (\n{}\n)",
                                conv_temps(self.temperature, self.scale)
                            ), Application::GUI);
                        };
                        if ui.selectable_value(&mut self.scale, Scale::Kelvin, "Kelvin").clicked() {
                            let _ = write_to_file( &format!(
                                "Temperature converted (\n{}\n)",
                                conv_temps(self.temperature, self.scale)
                            ), Application::GUI);
                        };
                        if ui.selectable_value(&mut self.scale, Scale::Fahrenheit, "Fahrenheit").clicked() {
                            let _ = write_to_file( &format!(
                                "Temperature converted (\n{}\n)",
                                conv_temps(self.temperature, self.scale)
                            ), Application::GUI);
                        };
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("Input Temperature");
                    ui.add(
                        DragValue::new(&mut self.temperature)
                        .speed(0.01)
                        .range(-9999.99..=9999.99)
                    );
                });
                ui.label(RichText::new(conv_temps(self.temperature, self.scale)).color(Color32::from_rgb(110, 255, 110)));
            }

            if (self.page == Page::Zip) {
                ui.heading("Zip Lookup");
                ui.horizontal(|ui| {
                    ui.label("Input Zip");
                    ui.text_edit_singleline(&mut self.zip);
                });
                if ui.button("Go!").clicked() {
                    get_temps_from_zip(&self.zip, ctx.clone(), self.tx.clone());
                }
                ui.label(RichText::new(&self.zipout).color(Color32::from_rgb(110, 255, 110)));
            }

            if (self.page == Page::Hist) {
                ui.heading("History");
                if ui.button(RichText::new("Update").color(Color32::from_rgb(110, 255, 110))).clicked() {
                    self.history = match read_from_file() {
                        Ok(t) => {
                            let _ = write_to_file(&"History accesed".to_string(), Application::GUI);
                            t
                        },
                        Err(e) => e.to_string()
                    }
                }
                ScrollArea::vertical().show(ui, |ui| {
                    ui.label(&self.history);
                });
            }
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
        let _ = write_to_file(&format!("Temperature retrieved by ZIP code (\n{}\n)", output).to_string(), Application::GUI);
        let _ = tx.send(output);
        ctx.request_repaint();
    });
}
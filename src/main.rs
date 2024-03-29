#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod word;

use eframe::egui;
use egui::*;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Wordle",
        options,
        Box::new(|_cc| Box::<Content>::default()),
    )
}

struct Content {
    text: String,
    last_key: Option<Key>,
    word: Result<String, ()>,
}

impl Default for Content {
    fn default() -> Self {
        Self {
            text: String::from(""),
            last_key: None,
            word: word::random_word(5)
        }
    }
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Press Keys");
            if ui.button("Clear").clicked() {
                self.text.clear();
            }
            ScrollArea::vertical()
                .auto_shrink(false)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.label(&self.text);
                });

            let keys_pressed = &ctx.input(|i| i.keys_down.clone());

            for key in keys_pressed {
                // Debounce to ignore same key presses
                if let Some(last_key) = self.last_key {
                    if &last_key != key {
                        println!("diff key from last");
                        println!("{:?}", self.word.as_ref().unwrap());
                    }
                } else {
                    // first key press
                }

                self.last_key = Some(key.to_owned());
            }

            // if ctx.input(|i| i.key_pressed(Key::)) {
            //
            // }
            //
            // if ctx.input(|i| i.key_pressed(Key::A)) {
            //     self.text.push_str("\nPressed");
            // }
            // if ctx.input(|i| i.key_down(Key::A)) {
            //     self.text.push_str("\nHeld");
            //     ui.ctx().request_repaint(); // make sure we note the holding.
            // }
            // if ctx.input(|i| i.key_released(Key::A)) {
            //     self.text.push_str("\nReleased");
            // }
        });
    }
}
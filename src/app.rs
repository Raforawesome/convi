#![allow(unused_imports)]
use eframe::egui;
use rfd::FileDialog;
use std::{path::PathBuf, fs::File};

#[derive(Default)]
pub struct App {
	input_path: PathBuf,
	output_path: String,
	// output_format: String,
	// log: Vec<String>
}

impl App {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
		cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		// egui::TopBottomPanel::new(egui::panel::TopBottomSide::Bottom, 2).show(ctx, |ui| {
		// 	ui.heading("Status:");
		// 	for msg in self.log.iter() {
		// 		ui.label(msg);
		// 	}
		// });
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Convi");
			ui.label("Files");
			ui.indent(2, |ui| {
				ui.label("Input File:");
				ui.text_edit_singleline(&mut self.input_path.clone().to_str().unwrap());
				if ui.button("Set Input File").clicked() {
					if let Some(p) = FileDialog::new().pick_file() {
						self.input_path = p;
					}
				}

				ui.label("Output File:");
				ui.text_edit_singleline(&mut self.output_path);
			});

			ui.add_space(30_f32);
			ui.heading("Output");
			// ui.label("Format");
			// ui.text_edit_singleline(&mut self.output_format);
			
			ui.add_space(5_f32);
			if ui.button("Convert").clicked() {
				println!("Spawning task with {} and {}",
			&self.input_path.to_str().unwrap(), &self.output_path);
				let _ = std::process::Command::new("ffmpeg")
					.arg("-i")
					.arg(self.input_path.to_str().unwrap())
					.arg(&self.output_path)
					.output();
			}
		});
    }
}
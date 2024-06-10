mod pbkdf2_aes;
use pbkdf2_aes::{decrypt, encrypt};

pub struct MyApp {
    input: String,
    password: String,
    output: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            password: String::new(),
            output: String::new(),
        }
    }
}

impl MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_visuals(egui::Visuals::dark());

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.heading("AES256-GCM via PBKDF2");
                        egui::Grid::new("entry_ui")
                            .num_columns(2)
                            .min_col_width(60.0)
                            .show(ui, |ui| {
                                ui.label("Input");
                                ui.add(
                                    egui::TextEdit::multiline(&mut self.input)
                                        .desired_rows(20)
                                        .desired_width(f32::INFINITY),
                                );
                                ui.end_row();

                                ui.label("Password");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.password)
                                        .desired_width(f32::INFINITY),
                                );
                                ui.end_row();
                            });

                        ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                            if ui.button("Encrypt").clicked() {
                                match encrypt(&self.password, &self.input) {
                                    Ok(result) => {
                                        self.output = result;
                                        self.input.clear();
                                        self.password.clear();
                                    }
                                    Err(err) => {
                                        ui.label(format!("Encryption failed: {}", err));
                                    }
                                }
                            }

                            if ui.button("Decrypt").clicked() {
                                match decrypt(&self.password, &self.input) {
                                    Ok(result) => {
                                        self.output = result;
                                        self.input.clear();
                                        self.password.clear();
                                    }
                                    Err(err) => {
                                        ui.label(format!("Decryption failed: {}", err));
                                    }
                                }
                            }
                        });
                        egui::Grid::new("output_ui")
                            .num_columns(2)
                            .min_col_width(60.0)
                            .show(ui, |ui| {
                                ui.label("Output");
                                ui.add(
                                    egui::TextEdit::multiline(&mut self.output)
                                        .desired_rows(20)
                                        .desired_width(f32::INFINITY),
                                );
                                ui.end_row();
                            });
                    });
            });
        });
    }
}

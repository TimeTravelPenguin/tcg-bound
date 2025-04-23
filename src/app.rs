/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    pub value: u32,
    pub max_value: u32,
    pub rows: u16,
    pub cols: u16,
    pub pages: u16,
}

impl Default for App {
    fn default() -> Self {
        Self {
            value: 1,
            max_value: 300,
            rows: 3,
            cols: 3,
            pages: 20,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("TCG Bound");

            let ui_builder = egui::UiBuilder::new();
            ui.scope_builder(ui_builder, |ui| {
                egui::Grid::new("central_grid_opts")
                    .num_columns(2)
                    .spacing([10.0, 10.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Rows: ");
                        ui.add(egui::DragValue::new(&mut self.rows).range(1..=u16::MAX));

                        ui.end_row();

                        ui.label("Columns: ");
                        ui.add(egui::DragValue::new(&mut self.cols).range(1..=u16::MAX));

                        ui.end_row();

                        ui.label("Pages: ");
                        ui.add(egui::DragValue::new(&mut self.pages).range(1..=u16::MAX));

                        ui.end_row();

                        ui.label("Max Value: ");
                        ui.add(egui::DragValue::new(&mut self.max_value).range(1..=u32::MAX));

                        ui.end_row();
                    });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Card Number: ");
                    ui.add(egui::Slider::new(&mut self.value, 1..=self.max_value));

                    ui.end_row();

                    if ui.button("  +1  ").clicked() {
                        self.value = self.value.saturating_add(1).min(self.max_value);
                    }

                    if ui.button("  -1  ").clicked() {
                        self.value = self.value.saturating_sub(1).max(1);
                    }

                    ui.end_row();
                })
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.label("Card binder location: ");

                let page_slots = (self.rows * self.cols) as u32;
                let page = self.value.div_euclid(page_slots) + 1;
                ui.horizontal(|ui| {
                    ui.label("Page: ");
                    ui.label(format!("{}/{}", page, self.pages));
                });

                ui.horizontal(|ui| {
                    ui.label("Slot: ");
                    ui.label(format!(
                        "{}/{}",
                        1 + (self.value - 1) % page_slots, // shift to 1-based index
                        page_slots
                    ));
                });
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                use egui::special_emojis::GITHUB;
                ui.add(egui::github_link_file!(
                    "https://github.com/TimeTravelPenguin/tcg-bound/blob/main/",
                    format!("{GITHUB} Source code")
                ));

                egui::warn_if_debug_build(ui);
            });
        });
    }
}

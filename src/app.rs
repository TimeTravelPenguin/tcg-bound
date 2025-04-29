use egui::{Align, Button, Label, Layout, Separator, Slider, TextEdit, Ui, Vec2, Widget};
use egui_extras::{Column, TableBuilder};
use egui_flex::{item, Flex, FlexAlignContent};

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

    pub fn page_slots(&self) -> u32 {
        (self.rows * self.cols) as u32
    }

    pub fn current_page(&self) -> u32 {
        self.value.div_euclid(self.page_slots()) + 1
    }

    pub fn current_slot(&self) -> u32 {
        self.value % self.page_slots() + 1 // Shift to 1-based index
    }

    pub fn current_slot_row(&self) -> u32 {
        (self.value % self.page_slots()) / self.cols as u32
    }

    pub fn current_slot_col(&self) -> u32 {
        (self.value % self.page_slots()) % self.cols as u32
    }
}

fn draw_table(ui: &mut Ui, app: &mut App) {
    TableBuilder::new(ui)
        .column(Column::auto().resizable(false))
        .column(Column::remainder())
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.horizontal(|ui| {
                    ui.heading("Settings");
                    ui.add_space(20.0);
                });
            });

            header.col(|ui| {
                ui.heading("Value");
            });
        })
        .body(|mut body| {
            body.row(20.0, |mut row| {
                row.col(|ui| {
                    ui.label("Rows");
                });
                row.col(|ui| {
                    ui.add(egui::DragValue::new(&mut app.rows).range(1..=u16::MAX));
                });
            });

            body.row(20.0, |mut row| {
                row.col(|ui| {
                    ui.label("Columns");
                });
                row.col(|ui| {
                    ui.add(egui::DragValue::new(&mut app.cols).range(1..=u16::MAX));
                });
            });

            body.row(20.0, |mut row| {
                row.col(|ui| {
                    ui.label("Pages");
                });
                row.col(|ui| {
                    ui.add(egui::DragValue::new(&mut app.pages).range(1..=u16::MAX));
                });
            });

            body.row(20.0, |mut row| {
                row.col(|ui| {
                    ui.label("Max Value");
                });
                row.col(|ui| {
                    ui.add(egui::DragValue::new(&mut app.max_value).range(1..=u32::MAX));
                });
            });
        });
}

fn card_table(id: &str, ui: &mut Ui, app: &mut App, is_left_page: bool) {
    let card_ratio = Vec2::new(2.5, 3.5);

    TableBuilder::new(ui)
        .id_salt(id)
        .vscroll(false)
        .columns(Column::auto().resizable(false), app.cols as usize)
        .body(|mut body| {
            body.rows(20.0, app.rows.into(), |mut row| {
                let is_current_page = app.current_page() % 2 == is_left_page as u32;

                for col in 0..app.cols {
                    let selected = is_current_page
                        && app.current_slot_row() == row.index() as u32
                        && app.current_slot_col() == col as u32;

                    let dbg_label = format!("Row: {}, Col: {}", row.index(), col);

                    let dbg_label = cfg!(debug_assertions)
                        .then(|| dbg_label)
                        .unwrap_or_default();

                    row.col(|ui| {
                        Button::new(dbg_label)
                            .min_size(card_ratio * 20.0)
                            .selected(selected)
                            .ui(ui);
                    });
                }
            });
        });
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

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                use egui::special_emojis::GITHUB;
                ui.add(egui::github_link_file!(
                    "https://github.com/TimeTravelPenguin/tcg-bound/blob/main/",
                    format!("{GITHUB} Source code")
                ));

                egui::warn_if_debug_build(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TCG Bound");

            draw_table(ui, self);

            ui.separator();

            Flex::vertical().show(ui, |flex| {
                flex.add(item().shrink(), Label::new("Card Number:"));
                flex.add(item(), Slider::new(&mut self.value, 1..=self.max_value));

                flex.add_flex(
                    item().grow(1.0),
                    Flex::horizontal()
                        .width(200.0)
                        .align_content(FlexAlignContent::Stretch),
                    |flex| {
                        if flex
                            .add(item().min_height(20.0), Button::new("-1"))
                            .clicked()
                        {
                            self.value = self.value.saturating_sub(1).max(1);
                        }

                        if flex
                            .add(item().min_height(20.0), Button::new("+1"))
                            .clicked()
                        {
                            self.value = self.value.saturating_add(1).min(self.max_value);
                        }
                    },
                );

                if flex.add(item().grow(1.0), Button::new("Reset")).clicked() {
                    self.value = 1;
                }
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.label("Card binder location: ");

                ui.horizontal(|ui| {
                    ui.label("Page: ");
                    ui.label(format!("{}/{}", self.current_page(), self.pages));
                });

                ui.horizontal(|ui| {
                    ui.label("Slot: ");
                    ui.label(format!("{}/{}", self.current_slot(), self.page_slots()));
                });
            });

            ui.horizontal(|ui| {
                card_table("visual_table_left", ui, self, true);
                ui.separator();
                card_table("visual_table_right", ui, self, false);
            });
        });
    }
}

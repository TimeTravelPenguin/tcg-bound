use std::num::NonZeroU32;

use crate::{
    binder::{Binder, BinderSlot},
    card_number::{CardNumber, SlotIndex},
};
use egui::{Button, Label, Slider, Ui, Vec2, Widget};
use egui_extras::{Column, TableBuilder};
use egui_flex::{item, Flex};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    pub value: CardNumber,
    pub max_value: NonZeroU32,
    pub binder: Binder,
}

impl Default for App {
    fn default() -> Self {
        Self {
            value: CardNumber::try_new(1, 100).expect("Default value should be 1"),
            max_value: NonZeroU32::new(100).expect("Default value should be 100"),
            binder: Binder::new(3, 3, 20),
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

fn settings_table(ui: &mut Ui, app: &mut App) {
    ui.collapsing("Settings", |ui| {
        TableBuilder::new(ui)
            .column(Column::auto().resizable(false))
            .column(Column::remainder())
            // .header(20.0, |mut header| {
            //     header.col(|ui| {
            //         ui.horizontal(|ui| {
            //             ui.heading("Settings");
            //             ui.add_space(20.0);
            //         });
            //     });
            //
            //     header.col(|ui| {
            //         ui.heading("Value");
            //     });
            // })
            .body(|mut body| {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("Rows");
                    });

                    let mut rows = app.binder.rows();
                    row.col(|ui| {
                        if ui
                            .add(egui::DragValue::new(&mut rows).range(1..=u16::MAX))
                            .changed()
                        {
                            let prev_state = app.binder;
                            if app.binder.set_rows(rows).is_err() {
                                app.binder = prev_state;
                            }
                        }
                    });
                });

                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("Columns");
                    });

                    let mut cols = app.binder.cols();
                    row.col(|ui| {
                        if ui
                            .add(egui::DragValue::new(&mut cols).range(1..=u16::MAX))
                            .changed()
                        {
                            let prev_state = app.binder;
                            if app.binder.set_cols(cols).is_err() {
                                app.binder = prev_state;
                            }
                        }
                    });
                });

                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("Pages");
                    });

                    let mut pages = app.binder.pages();
                    row.col(|ui| {
                        if ui
                            .add(egui::DragValue::new(&mut pages).range(1..=u16::MAX))
                            .changed()
                        {
                            let prev_state = app.binder;
                            if app.binder.set_pages(pages).is_err() {
                                app.binder = prev_state;
                            }
                        }
                    });
                });

                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label("Max Value");
                    });
                    row.col(|ui| {
                        ui.add(egui::DragValue::new(&mut app.max_value).range(1..=u16::MAX));
                    });
                });
            });
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CardTable {
    Left,
    Right,
}

fn card_table(id: &str, ui: &mut Ui, app: &mut App, page_side: CardTable) {
    let card_ratio = Vec2::new(2.5, 3.5);

    let binder_rows = app.binder.rows();
    let binder_cols = app.binder.cols();

    TableBuilder::new(ui)
        .id_salt(id)
        .vscroll(false)
        .columns(Column::auto().resizable(false), binder_cols as usize)
        .body(|body| {
            body.rows(20.0, binder_rows as usize, |mut row| {
                for col in 0..binder_cols {
                    let page_offset = app.value.to_index().get() / (binder_rows * binder_cols);
                    let current_cell_index = SlotIndex::new(
                        (row.index() as u32 * binder_cols + col)
                            + page_offset * binder_rows * binder_cols,
                    );

                    // Check if the current slot is the selected one
                    // The selected slot is the one that matches the current card number
                    // and is on the correct page
                    let current_slot = BinderSlot::from_card_number(&app.binder, app.value);

                    // Check if the current user-selected card slot is on the correct page
                    let correct_page = match page_side {
                        // Page 1 starts on the left side, Page 2 starts on the right side,
                        // so the modulo seems to be inverted
                        CardTable::Left => page_offset % 2 == 1,
                        CardTable::Right => page_offset % 2 == 0,
                    };

                    let current_slot_selected =
                        correct_page && current_slot.index() == current_cell_index;

                    let dbg_label = format!(
                        "Row: {}, Col: {}\nPage: {}\nCorrect page: {}\nSelected: {}",
                        row.index(),
                        col,
                        current_slot.page(),
                        correct_page,
                        current_slot_selected
                    );

                    let dbg_label = cfg!(debug_assertions)
                        .then(|| dbg_label)
                        .unwrap_or_default();

                    row.col(|ui| {
                        Button::new(dbg_label)
                            .min_size(card_ratio * 20.0)
                            .selected(current_slot_selected)
                            .ui(ui);
                    });
                }
            });
        });
}

impl eframe::App for App {
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

            settings_table(ui, self);

            ui.separator();

            Flex::vertical().grow_items(1.0).show(ui, |flex| {
                flex.add(
                    item().shrink().align_self(egui_flex::FlexAlign::Start),
                    Label::new("Card Number:"),
                );

                let slider_width = 100.0;
                let mut slider_value = self.value.get();
                if flex
                    .add(
                        item().grow(1.0).min_width(slider_width),
                        Slider::new(&mut slider_value, 1..=self.max_value.get()),
                    )
                    .changed()
                {
                    if let Some(new_value) = CardNumber::try_new(slider_value, self.max_value.get())
                    {
                        self.value = new_value;
                    }
                }

                flex.add_flex(
                    item(),
                    Flex::horizontal().grow_items(1.0).w_full(),
                    |flex| {
                        if flex
                            .add(item().min_height(30.0), Button::new("-10"))
                            .clicked()
                        {
                            self.value = CardNumber::try_new(
                                self.value.get().saturating_sub(10),
                                self.max_value.get(),
                            )
                            .unwrap_or(self.value);
                        }

                        if flex
                            .add(item().min_height(30.0), Button::new("+10"))
                            .clicked()
                        {
                            self.value = CardNumber::try_new(
                                self.value.get().saturating_add(10),
                                self.max_value.get(),
                            )
                            .unwrap_or(self.value);
                        }
                    },
                );

                flex.add_flex(
                    item(),
                    Flex::horizontal().grow_items(1.0).w_full(),
                    |flex| {
                        if flex
                            .add(item().min_height(30.0), Button::new("-1"))
                            .clicked()
                        {
                            self.value = CardNumber::try_new(
                                self.value.get().saturating_sub(1),
                                self.max_value.get(),
                            )
                            .unwrap_or(self.value);
                        }

                        if flex
                            .add(item().min_height(30.0), Button::new("+1"))
                            .clicked()
                        {
                            self.value = CardNumber::try_new(
                                self.value.get().saturating_add(1),
                                self.max_value.get(),
                            )
                            .unwrap_or(self.value);
                        }
                    },
                );

                if flex.add(item(), Button::new("Reset")).clicked() {
                    self.value = CardNumber::try_new(1, self.max_value.get())
                        .expect("Default value should be 1");
                }
            });

            ui.separator();

            let slot = BinderSlot::from_index(&self.binder, self.value.to_index());

            ui.vertical(|ui| {
                ui.label("Card binder location: ");

                ui.horizontal(|ui| {
                    ui.label("Page: ");
                    ui.label(format!("{}/{}", slot.page(), self.binder.pages()));
                });

                ui.horizontal(|ui| {
                    ui.label("Slot: "); // Slot on the current page
                    ui.label(format!(
                        "{}/{}",
                        slot.index().get() % self.binder.total_page_slots() + 1,
                        self.binder.total_page_slots()
                    ));
                });
            });

            ui.horizontal(|ui| {
                card_table("visual_table_left", ui, self, CardTable::Left);
                ui.separator();
                card_table("visual_table_right", ui, self, CardTable::Right);
            });

            // Flex::horizontal().show(ui, |flex| {
            //     flex.add_flex(item(), Flex::vertical(), |flex| {
            //         flex.add_ui(item(), |ui| {
            //             card_table("visual_table_left", ui, self, CardTable::Left);
            //         });
            //
            //         flex.add(
            //             item().shrink().align_self(egui_flex::FlexAlign::Start),
            //             Label::new("Left Page"),
            //         );
            //     });
            //
            //     flex.add_ui(item().align_self_content(egui::Align2::CENTER_TOP), |ui| {
            //         ui.separator();
            //     });
            //
            //     flex.add_flex(item(), Flex::vertical(), |flex| {
            //         flex.add_ui(item(), |ui| {
            //             card_table("visual_table_right", ui, self, CardTable::Right);
            //         });
            //
            //         flex.add(
            //             item().shrink().align_self(egui_flex::FlexAlign::Start),
            //             Label::new("Right Page"),
            //         );
            //     });
            // });

            ui.allocate_space(ui.available_size());
        });
    }
}

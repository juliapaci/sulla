use eframe::egui;
use egui_dock::{DockArea, DockState, NodeIndex, Style};

use std::path::PathBuf;
use egui_file_dialog::FileDialog;

struct TabViewer<'a> {
    state: &'a mut SharedState
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab.as_str() {
            "Assets" => self.file_tab(ui),
            _ => {
                ui.label(format!("Empty {tab} contents"));
            }
        }
    }
}

impl TabViewer<'_> {
    fn file_tab(&mut self, ui: &mut egui::Ui) {
        if ui.add_sized([ui.min_rect().width(), 10.], egui::Button::new("Add file")).clicked() {
            self.state.file.file_dialog.select_file();
        }

        if let Some(path) = self.state.file.file_dialog.update(ui.ctx()).selected() {
            self.state.file.selected_file = Some(path.to_path_buf());
        }

        ui.add_space(15.0);

        use egui_extras::{TableBuilder, Column};

        let text_height = egui::TextStyle::Body
            .resolve(ui.style())
            .size
            .max(ui.spacing().interact_size.y);

        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .sense(egui::Sense::click())
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::initial(30.0).at_least(30.0).clip(true))
            .column(Column::initial(100.0).clip(true))
            .column(Column::auto().clip(true))
            .column(Column::auto().clip(true))
            .column(Column::remainder())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.strong("Used");
                    });
                });
                header.col(|ui| {
                    ui.strong("Name");
                });
                header.col(|ui| {
                    ui.strong("Size");
                });
                header.col(|ui| {
                    ui.strong("Type");
                });
                header.col(|ui| {
                    ui.strong("Thumbnail");
                });
            })
            .body(|body| {
                body.rows(
                    text_height,
                    10, |mut row| {
                        row.col(|ui| {
                            ui.centered_and_justified(|ui| {
                                ui.checkbox(&mut false, "");
                            });
                        });
                        row.col(|ui| {
                            ui.label("test 1");
                        });
                        row.col(|ui| {
                            ui.label("test 2");
                        });
                        row.col(|ui| {
                            ui.label("test 3");
                        });
                        row.col(|ui| {
                            ui.label("test 4");
                        });
                    })
            });
    }
}

// keeps track of file state
#[derive(Default)]
struct FileState {
    file_dialog: FileDialog,
    selected_file: Option<PathBuf>
}

// shared state between SullaState (app) and TabViewer (tabs)
#[derive(Default)]
struct SharedState {
    file: FileState
}

struct SullaState {
    tree: DockState<String>,
    state: SharedState,
}

impl Default for SullaState {
    fn default() -> Self {
        let mut tree = DockState::new(vec!["Timeline".to_owned()]);

        let [timeline, assets] =
            tree.main_surface_mut()
                .split_left(NodeIndex::root(), 0.25, vec!["Hierarchy".to_owned(), "Assets".to_owned()]);
        let [_, _inspector] =
            tree.main_surface_mut()
                .split_below(assets, 0.5, vec!["Inspector".to_owned()]);
        let [_, _player] =
            tree.main_surface_mut()
                .split_above(timeline, 0.5, vec!["Player".to_owned(), "Scene".to_owned()]);

        Self {
            tree,
            state: Default::default()
        }
    }
}

impl eframe::App for SullaState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut tv = TabViewer {
            state: &mut self.state
        };

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tv);
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "sulla",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::<SullaState>::default())
    )
}

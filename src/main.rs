use eframe::egui;
use egui_dock::{DockArea, DockState, NodeIndex, Style};

use egui_file::FileDialog;
use std::{
  ffi::OsStr,
  path::{Path, PathBuf},
};

struct TabViewer {
    state: SharedState
}

impl egui_dock::TabViewer for TabViewer {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab.as_str() {
            "File" => self.file_tab(ui),
            _ => {
                ui.label(format!("Empty {tab} contents"));
            }
        }
    }
}

impl TabViewer {
    fn file_tab(&mut self, ui: &mut egui::Ui) {
        if ui.button("open file").clicked() {
            self.state.file = true;
        }
    }
}

impl Default for TabViewer {
    fn default() -> Self {
        Self {
            state: Default::default()
        }
    }
}

#[derive(Default, Clone, Copy)]
struct SharedState {
    file: bool
}

// keeps track of file state
#[derive(Default)]
struct FileState {
    opened_file: Option<PathBuf>,
    open_file_dialog: Option<FileDialog>,
}

struct SullaState {
    tree: DockState<String>,
    state: SharedState,

    file: FileState
}

impl Default for SullaState {
    fn default() -> Self {
        let mut tree = DockState::new(vec!["Timeline".to_owned()]);

        let [timeline, assets] =
            tree.main_surface_mut()
                .split_left(NodeIndex::root(), 0.25, vec!["Assets".to_owned(), "File".to_owned()]);
        let [_, _inspector] =
            tree.main_surface_mut()
                .split_below(assets, 0.5, vec!["Inspector".to_owned()]);
        let [_, _player] =
            tree.main_surface_mut()
                .split_above(timeline, 0.5, vec!["Player".to_owned(), "Scene".to_owned()]);

        Self {
            tree,
            state: Default::default(),

            file: Default::default()
        }
    }
}

impl eframe::App for SullaState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let tv = &mut TabViewer {
                state: self.state
            };

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, tv);

        self.state = tv.state;

        if self.state.file {
            // Show only files with the extension "txt".
            let filter = Box::new({
                let ext = Some(OsStr::new("txt"));
                move |path: &Path| -> bool { path.extension() == ext }
            });
            let mut dialog = FileDialog::open_file(self.file.opened_file.clone()).show_files_filter(filter);
            dialog.open();
            self.file.open_file_dialog = Some(dialog);

            if let Some(dialog) = &mut self.file.open_file_dialog {
                if dialog.show(ctx).selected() {
                    if let Some(file) = dialog.path() {
                        self.file.opened_file = Some(file.to_path_buf());
                    }
                }
            }
        }
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "sulla",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::<SullaState>::default())
    )
}

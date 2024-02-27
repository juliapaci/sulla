use eframe::egui;
use egui::{Frame, Ui};

use egui_dock::{DockArea, DockState, NodeIndex, Style};

struct TabViewer;

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

        }
    }
}

struct SullaApp {
    tree: DockState<String>,
}

impl Default for SullaApp {
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
            tree
        }
    }
}

impl eframe::App for SullaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::central_panel(&ctx.style()).inner_margin(0.))
            .show(ctx, |ui| {
                DockArea::new(&mut self.tree)
                    .style(Style::from_egui(ctx.style().as_ref()))
                    .show(ctx, &mut TabViewer {});

                if ui
                    .selectable_label(match self.tree.find_active_focused() {
                        Some(focused) => focused.1 == "File",
                        None => false
                    }, "open file")
                    .clicked() {
                    println!("test");
                }
            });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "sulla",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::<SullaApp>::default())
    )
}

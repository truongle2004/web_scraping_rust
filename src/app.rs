use crate::{components, integrate};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)]
    form_value: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    #[serde(skip)]
    is_enabled_left_side_panel: bool,
    #[serde(skip)]
    is_pressed_enter_fetching_url: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            form_value: "Enter the url here!".to_owned(),
            value: 2.7,
            is_enabled_left_side_panel: true,
            is_pressed_enter_fetching_url: false,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
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

        // TODO
        // components::top_bottom_panel::view(ui);

        components::left_side_panel::left_side_panel_view(
            &ctx,
            self.is_enabled_left_side_panel,
            || {
                self.is_enabled_left_side_panel = false;
            },
        );

        components::central_panel::central_panel_view(&ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("enter url here");
                ui.text_edit_singleline(&mut self.form_value);
            });

            if ui.button("call python function").clicked() {
                integrate::integrate::greet_test();
            }
        });

        if self.is_pressed_enter_fetching_url {
            let url = self.form_value.clone();
            println!("fetching url: {}", url);
        }
    }
}

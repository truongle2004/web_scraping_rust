pub fn view(ui: &mut egui::Ui) {
    egui::TopBottomPanel::top("top panel").show_inside(ui, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Home").clicked() {
                println!("Home button clicked");
            }
            if ui.button("Settings").clicked() {
                println!("Settings button clicked");
            }
            if ui.button("Profile").clicked() {
                println!("Profile button clicked");
            }
        });
    });
}

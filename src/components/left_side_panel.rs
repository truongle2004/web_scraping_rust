pub fn left_side_panel_view<F>(ctx: &egui::Context, enabled: bool, mut close_side_panel: F)
where
    F: FnMut(),
{
    if !enabled {
        return;
    }

    egui::SidePanel::left("left side panel")
        .min_width(300.00)
        .show(ctx, |ui| {
            if ui.button("close side panel").clicked() {
                close_side_panel();
            }
        });
}

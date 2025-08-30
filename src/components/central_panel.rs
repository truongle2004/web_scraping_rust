pub fn central_panel_view<F>(ctx: &egui::Context, mut children: F)
where
    F: FnMut(&mut egui::Ui),
{
    egui::CentralPanel::default().show(ctx, |ui| {
        children(ui);
    });
}

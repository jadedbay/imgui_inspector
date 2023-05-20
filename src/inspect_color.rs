use crate::InspectColor;

impl InspectColor for [f32; 3] {
    fn inspect_color<'a>(&mut self, ui: &'a imgui::Ui, label: &str) -> bool {
        ui.color_edit3(label, self)
    }
}
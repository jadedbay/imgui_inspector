mod inspect_numeric;

pub trait ImguiInspect {
    fn imgui_inspect<'a>(&mut self, ui: &'a imgui::Ui) -> bool;
}

pub trait InspectNumeric {
    fn inspect_drag<'a>(&mut self, ui: &'a imgui::Ui, label: &str, min: f32, max: f32) -> bool;
    fn inspect_slider<'a>(&mut self, ui: &'a imgui::Ui, label: &str, min: f32, max: f32) -> bool;
}
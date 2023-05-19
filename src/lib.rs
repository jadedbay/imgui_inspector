mod inspect_numeric;
mod inspect_custom;

pub trait ImguiInspect {
    fn imgui_inspect<'a>(&mut self, ui: &'a imgui::Ui) -> bool;
}

pub trait InspectNumeric {
    fn inspect_drag<'a>(&mut self, ui: &'a imgui::Ui, label: &str, min: f32, max: f32, speed: f32) -> bool;
    fn inspect_slider<'a>(&mut self, ui: &'a imgui::Ui, label: &str, min: f32, max: f32) -> bool;
}

pub trait InspectCustomVector {
    fn inspect_custom<'a>(&mut self, ui: &'a imgui::Ui, label: &str, min: f32, max: f32, speed: f32) -> bool;
}
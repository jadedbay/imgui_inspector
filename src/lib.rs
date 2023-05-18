mod inspect_numeric;

pub trait ImguiInspect {
    fn imgui_inspect<'a>(&mut self, ui: &'a imgui::Ui) -> bool;
}

pub trait InspectNumeric {
    fn inspect_drag<'a>(&mut self, ui: &'a imgui::Ui, label: &str) -> bool;
    fn inspect_slider<'a>(&mut self, ui: &'a imgui::Ui, label: &str) -> bool;
}
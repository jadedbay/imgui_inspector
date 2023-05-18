mod inspect_numeric;

pub trait ImguiInspect {
    fn inspect(&mut self, ui: &mut imgui::Ui) -> bool;
}

pub trait InspectNumeric {
    fn inspect_drag<'a>(&mut self, ui: &'a imgui::Ui, label: &str) -> bool;
    fn inspect_slider<'a>(&mut self, ui: &'a imgui::Ui, label: &str) -> bool;
}
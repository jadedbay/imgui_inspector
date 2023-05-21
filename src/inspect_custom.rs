use crate::InspectCustomVector;

impl InspectCustomVector for cg::Vector3<f32> {
    fn inspect_custom<'a>(&mut self, ui: &'a imgui::Ui, label: &str, min: f32, max: f32, speed: f32) -> bool {
        let mut clamped = false;
        if min != max { clamped = true }
        
        let mut is_changed: Vec<bool> = Vec::new();
        
        let default_color = ui.clone_style().colors[imgui::StyleColor::Button as usize];
        let hover = ui.push_style_color(imgui::StyleColor::ButtonHovered, default_color);
        let clicked = ui.push_style_color(imgui::StyleColor::ButtonActive, default_color);

        let window_width = ui.content_region_avail()[0];
        let padding = ui.clone_style().window_padding;
        let text_size = ui.calc_text_size(label);
        let field_width = (window_width - (19.0 * 3.0) - (padding[0] * 2.0) - (text_size[0] + 56.0 - text_size[0])) / 3.0;

        ui.text(label);

        let width = ui.push_item_width(field_width);

        ui.same_line_with_spacing(0.0, 56.0 - text_size[0] + ui.clone_style().item_spacing[0]);

        ui.button_with_size(format!("x##{}",label), [19.0, 19.0]);
        
        if ui.is_item_hovered() {
            ui.set_mouse_cursor(Some(imgui::MouseCursor::ResizeEW));
        }

        if ui.is_item_active() && ui.io().mouse_down[0] {
            let delta = ui.io().mouse_delta[0] * speed;
            if (self.x + delta) < min && clamped { self.x = min; }
            else if (self.x + delta) > max && clamped { self.x = max; }
            else { self.x += delta; }
            is_changed.push(true);
        }

        ui.same_line_with_spacing(0.0, 0.0);
        
        is_changed.push(ui.input_float(format!("##x{}",label), &mut self.x).build());
        if clamped { self.x = self.x.max(min).min(max); }

        ui.same_line();

        ui.button_with_size(format!("y##{}",label), [19.0, 19.0]);
        if ui.is_item_hovered() {
            ui.set_mouse_cursor(Some(imgui::MouseCursor::ResizeEW));
        }
        if ui.is_item_active() && ui.io().mouse_down[0] {
            let delta = ui.io().mouse_delta[0] * speed;
            if (self.y + delta) < min && clamped { self.y = min; }
            else if (self.y + delta) > max && clamped { self.y = max; }
            else { self.y += delta; }
            is_changed.push(true);
        }
        ui.same_line_with_spacing(0.0, 0.0);
        is_changed.push(ui.input_float(format!("##y{}",label), &mut self.y).build());
        if clamped { self.y = self.y.max(min).min(max); }

        ui.same_line();

        ui.button_with_size(format!("z##{}",label), [19.0, 19.0]);
        if ui.is_item_hovered() {
            ui.set_mouse_cursor(Some(imgui::MouseCursor::ResizeEW));
        } 
        if ui.is_item_active() && ui.io().mouse_down[0] {
            let delta = ui.io().mouse_delta[0] * speed;
            if (self.z + delta) < min && clamped { self.z = min; }
            else if (self.z + delta) > max && clamped { self.z = max; }
            else { self.z += delta; }
            is_changed.push(true);
        }
        ui.same_line_with_spacing(0.0, 0.0);
        is_changed.push(ui.input_float(format!("##z{}",label), &mut self.z).build());
        if clamped { self.z = self.z.max(min).min(max); }
        
        width.end();
        hover.pop();
        clicked.pop();

        is_changed.iter().any(|&value| value == true)
    }
}
use crate::InspectNumeric;

macro_rules! impl_inspect_numeric {
    ($($t:ty),+) => {
        $(
            impl InspectNumeric for $t {
                fn inspect_drag<'a>(&mut self, ui: &'a imgui::Ui, label: &str) -> bool {
                    imgui::Drag::new(label)
                        .build(ui, self)
                }
                fn inspect_slider<'a>(&mut self, ui: &'a imgui::Ui, label: &str) -> bool {
                    ui.slider(label, -100.0 as $t, 100.0 as $t, self)
                }
            }
        )+
    }
}

impl_inspect_numeric!(
    f32, f64,
    i8, u8,
    i16, u16,
    i32, u32,
    i64, u64,
    isize, usize
);

macro_rules! impl_inspect_generic {
    ($c:ident::$vec:ident $fields:tt, $($t:ty),+) => {
        $(
            impl_inspect_generic!(@fields $c::$vec $fields, $t);
        )+
    };

    (@fields $c:ident::$vec:ident($($field:ident),*), $t:ty) => {
        impl InspectNumeric for $c::$vec<$t> {
            fn inspect_drag<'a>(&mut self, ui: &'a imgui::Ui, _label: &str) -> bool {
                let mut is_changed = false;

                $(
                    let value = imgui::Drag::new(stringify!($field)).build(ui, &mut self.$field);
                    if !is_changed { is_changed = value; }
                )*

                is_changed
            }
            fn inspect_slider<'a>(&mut self, ui: &'a imgui::Ui, _label: &str) -> bool {
                let mut is_changed = false;

                $(
                    let value = ui.slider(stringify!($field), -100.0 as $t, 100.0 as $t, &mut self.$field);
                    if !is_changed { is_changed = value; }
                )*

                is_changed
            }
        }
    }
}

impl_inspect_generic!(
    cg::Vector3(x, y, z), 
    f32, f64,
    i8, u8,
    i16, u16,
    i32, u32,
    i64, u64,
    isize, usize
);

impl_inspect_generic!(
    cg::Vector4(x, y, z, w), 
    f32, f64,
    i8, u8,
    i16, u16,
    i32, u32,
    i64, u64,
    isize, usize
);
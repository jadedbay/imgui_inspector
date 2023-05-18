use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};
use darling::FromField;

#[derive(Debug, FromField)]
#[darling(attributes(inspect), default)]
struct InspectAttribute {
    widget: Option<String>,
    min: f32,
    max: f32,
    hide: bool,
    speed: f32,
}

impl Default for InspectAttribute {
    fn default() -> Self {
        Self {
            widget: Some("drag".to_string()),
            min: 0.0,
            max: 100.0,
            hide: false,
            speed: 1.0,
        }
    }
}

#[proc_macro_derive(ImguiInspect, attributes(inspect))]
pub fn imgui_inspector_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;
    let fields = match ast.data {
        Data::Struct(ref data) => {
            match data.fields {
                syn::Fields::Named(ref fields) => &fields.named,
                syn::Fields::Unnamed(_) => panic!("Tuple structs are not supported"),
                syn::Fields::Unit => panic!("Unit structs are not supported"),
            }
        }
        _ => panic!("Only structs are supported"),
    };

    let field_code = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let name = &field_name.to_string();

        let attribute = InspectAttribute::from_field(field).unwrap();
        if attribute.hide { return quote! {} }

        let (min, max) = (attribute.min, attribute.max);
        let speed = attribute.speed;
        let imgui_widget = if let Some(widget) = attribute.widget {
            match widget.as_str() {
                "drag" => quote! { self.#field_name.inspect_drag(ui, #name, #min, #max), },
                "slider" => quote! { self.#field_name.inspect_slider(ui, #name, #min, #max), },
                _ => panic!("Invalid Widget! Field: {}.{}", name, field_name)
            }
        } else {
            quote! {}
        };

        quote! { #imgui_widget }
    });

    let output = quote! {
        impl ImguiInspect for #name {
            fn imgui_inspect<'a>(&mut self, ui: &'a imgui::Ui) -> bool {
                let is_changed = vec![#(#field_code)*];
                is_changed.iter().any(|&value| value == true)
            }
        }
    };
    output.into()
}
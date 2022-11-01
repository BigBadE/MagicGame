extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{DeriveInput, Ident, parse_macro_input, PathSegment, Type};
use syn::punctuated::Punctuated;
use syn::token::Colon2;

#[proc_macro_derive(JsonResource, attributes(ignore, require))]
pub fn json_implement(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let mut output = String::from(format!("pub fn __load(input: &{struct_name}, value: JsonValue) {{"));

    let fields =
        if let syn::Data::Struct(
            syn::DataStruct {
                fields: syn::Fields::Named(ref fields),
                ..
            }) = ast.data
        {
            fields
        } else {
            panic!("Only support Struct")
        };

    for field in fields.named.iter() {
        match &field.ty {
            Type::Path(TypePath) => {
                let segments = &TypePath.path.segments;
                let mut ignore = false;
                let mut required = false;
                for attribute in &field.attrs {
                    match combine(&attribute.path.segments).as_str() {
                        "ignore" => ignore = true,
                        "required" => required = true,
                        _ => {}
                    }
                }

                if !ignore {
                    let field_name = field.ident.as_ref().unwrap().to_string();

                    if required {
                        output += format!("if value.has_key(\"{}\") {{", field_name).as_str();
                    }
                    output += load_path(field_name,
                                        combine(segments).as_str()).as_str();
                    if required {
                        output += "}"
                    }
                }
            }
            _ => panic!("Unknown type on field {}, did you mean to ignore it?", field.ident.as_ref().unwrap()),
        }
    }

    output.push('}');
    output.parse().unwrap()
}

fn combine(segments: &Punctuated<PathSegment, Colon2>) -> String {
    let mut output = String::new();
    for segment in segments {
        output += (segment.ident.to_string() + "::").as_str();
    }

    return String::from(&output[0..output.len() - 2]);
}

fn load_path(field_name: String, found_type: &str) -> String {
    return match found_type {
        "u8" => format!("input.{} = value[\"{}\"].as_u8().expect(\"No field {}\")", field_name, field_name, field_name),
        _ => panic!("Unknown type {} for field {}", found_type, field_name)
    };
}
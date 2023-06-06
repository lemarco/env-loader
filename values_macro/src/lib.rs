use proc_macro::TokenStream;
use quote::quote;
use std::str::FromStr;

struct KeyValue {
    key: String,
    ty: String,
}

impl FromStr for KeyValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(());
        }
        let key = parts[0].trim().to_string();
        let ty = parts[1].trim().to_string();
        Ok(KeyValue { key, ty })
    }
}

#[proc_macro]
pub fn convert(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input = input.trim();
    let input = input.strip_prefix('{').unwrap_or(input);
    let input = input.strip_suffix('}').unwrap_or(input);
    let input = input.trim();

    let key_values: Vec<KeyValue> = input
        .split(',')
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<KeyValue>, _>>()
        .unwrap_or_else(|_| panic!("Invalid input: {:?}", input));

    let result = key_values.iter().map(|KeyValue { key, ty }| {
        let key_str = key.as_str();
        let value_enum = match ty.to_ascii_lowercase().as_str() {
            "int" | "integer" | "i32" => quote! { Value::Int },
            "str" | "string" => quote! { Value::Str },
            "long" | "i64" => quote! { Value::Long },
            "bool" | "boolean" => quote! { Value::Bool },
            _ => panic!("Unsupported type: {:?}", ty),
        };
        quote! { (#key_str, #value_enum) }
    });

    let output = quote! { [ #( #result ),* ] };

    output.into()
}

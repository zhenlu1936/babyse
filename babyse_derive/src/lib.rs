use proc_macro::TokenStream;

#[proc_macro_derive(BabySerialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    let start = input_str.find('{').unwrap() + 1;
    let end = input_str.rfind('}').unwrap();
    let fields_str = &input_str[start..end];

    let field_names: Vec<&str> = fields_str
        .split(',')
        .filter_map(|line| line.split(':').next())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let struct_name = input_str.split_whitespace().nth(1).unwrap();

    let fields_code: String = field_names
        .iter()
        .map(|name| format!("s.push_str(&format!(\"{name}: {{:?}}, \", self.{name}));",))
        .collect::<Vec<_>>()
        .join("\n");

    let code = format!(
        r#"
            impl BabySerialize for {struct_name} {{
                fn serialize(&self) -> String {{
                    let mut s = String::new();
                    {fields_code}
                    s
                }}
            }}
        "#,
    );

    code.parse().unwrap()
}

#[proc_macro_derive(BabyDeserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    let struct_name = input_str.split_whitespace().nth(1).unwrap();
    let start = input_str.find('{').unwrap() + 1;
    let end = input_str.rfind('}').unwrap();
    let fields_str = &input_str[start..end];

    let field_names: Vec<&str> = fields_str
        .split(',')
        .map(|s| s.trim().split(':').next().unwrap().trim())
        .filter(|s| !s.is_empty())
        .collect();

    let fields_code: String = field_names
        .iter()
        .map(|name| format!(
            "{name}: map.get(\"{name}\").expect(\"Field {name} missing\").parse().expect(\"Failed to parse {name}\")",
        ))
        .collect::<Vec<_>>()
        .join(", ");

    let code = format!(
        r#"
            impl BabyDeserialize for {struct_name} {{
                fn deserialize(s: &str) -> Self {{
                    let mut map = std::collections::HashMap::new();
                    for pair in s.split(", ") {{
                        let mut kv = pair.splitn(2, ": ");
                        if let (Some(k), Some(v)) = (kv.next(), kv.next()) {{
                            map.insert(k.trim(), v.trim());
                        }}
                    }}
                    Self {{ {fields_code} }} 
                }}
            }}
        "#
    );

    code.parse().unwrap()
}
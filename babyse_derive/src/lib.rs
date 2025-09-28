use proc_macro::TokenStream;

#[proc_macro_derive(BabySerialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    let brace_pos = input_str.find('{').unwrap();
    let before_brace = &input_str[..brace_pos].trim();
    let struct_name = before_brace.split_whitespace().last().unwrap();

    let start = brace_pos + 1;
    let end = input_str.rfind('}').unwrap();
    let fields_str = &input_str[start..end];

    let field_names: Vec<&str> = fields_str
        .split(',')
        .filter_map(|line| line.split(':').next())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let name_code: String = format!("s.push_str(&format!(\"{struct_name} \"));");

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
                    {name_code}
                    s.push_str("{{ ");
                    {fields_code}
                    s.pop();
                    s.pop();
                    s.push_str("}}");
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

    let brace_pos = input_str.find('{').unwrap();
    let before_brace = &input_str[..brace_pos].trim();
    let struct_name = before_brace.split_whitespace().last().unwrap();

    let start = brace_pos + 1;
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
                    let brace_start = s.find('{{').unwrap() + 1;
                    let brace_end = s.rfind('}}').unwrap();
                    let inner = &s[brace_start..brace_end].trim();

                    let mut map = std::collections::HashMap::new();
                    for pair in inner.split(", ") {{
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

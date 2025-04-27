use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DSL {
    pub types: HashMap<String, Type>,
    pub root: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum Type {
    Array(Array),
    Struct(Struct),
    Union(Union),
    Enum(Enum),
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Array {
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Struct {
    pub fields: HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Union {
    pub variants: HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Enum {
    pub variants: HashMap<String, bool>,
}

pub fn main(from: &str, to: &str, c: bool, cpp: bool, json_schema: bool) {
    let content = if from.ends_with(".json") {
        let json = std::fs::read_to_string(from).unwrap();
        serde_jsonc2::from_str::<DSL>(&json).unwrap()
    } else if from.ends_with(".yaml") {
        let yaml = std::fs::read_to_string(from).unwrap();
        serde_yaml::from_str::<DSL>(&yaml).unwrap()
    } else {
        panic!("Unsupported file extension: {}", from);
    };

    generate_code(&content, to, c, cpp, json_schema);
}

pub fn validate(dsl: &DSL) {
    fn is_builtin_type(name: &str) -> bool {
        name == "bool" || name == "int" || name == "float" || name == "string"
    }

    fn is_invalid_name(name: &str) -> bool {
        name == "void"
            || name == "true"
            || name == "false"
            || name == "null"
            || name == "struct"
            || name == "union"
            || name == "enum"
            || name == "type"
        // TODO: add C/C++ keywords
    }

    fn validate_name(name: &str) {
        if name
            .chars()
            .all(|c| (c.is_alphanumeric() && c.is_ascii()) || c == '_')
            && !name.is_empty()
            && !is_invalid_name(name)
        {
            return;
        }
        panic!("Invalid type name: {}", name);
    }

    fn validate_type_name(name: &str) {
        validate_name(name);
        if is_builtin_type(name) {
            panic!("Builtin type name: {}", name);
        }
    }

    fn validate_field(dsl: &DSL, name: &str, field: &str) {
        validate_type_name(name);
        validate_name(field);
        if !is_builtin_type(field) && !dsl.contains_key(field) {
            panic!("Unknown type: {}", field);
        }
    }

    fn validate_type(dsl: &DSL, name: &str, ty: &Type) {
        match ty {
            Type::Array(a) => {
                validate_type_name(name);
                validate_name(&a.ty);
                if !is_builtin_type(&a.ty) && !dsl.contains_key(&a.ty) {
                    panic!("Unknown type: {}", a.ty);
                }
            }
            Type::Struct(s) => {
                validate_type_name(name);
                s.fields
                    .iter()
                    .for_each(|(name, field)| validate_field(dsl, name, field));
            }
            Type::Union(u) => {
                validate_type_name(name);
                u.variants.iter().for_each(|(name, variant)| {
                    validate_name(name);
                    validate_name(variant);
                    if !is_builtin_type(variant) && !dsl.contains_key(variant) {
                        panic!("Unknown type: {}", variant);
                    }
                });
            }
            Type::Enum(e) => {
                validate_type_name(name);
                e.variants
                    .iter()
                    .for_each(|(name, _)| validate_type_name(name));
            }
        }
    }

    dsl.iter()
        .for_each(|(name, ty)| validate_type(dsl, name, ty));
}

pub fn generate_code(dsl: &DSL, to: &str, c: bool, cpp: bool, json_schema: bool) {
    validate(dsl);

    std::fs::create_dir_all(to).unwrap();

    if c || cpp {
        let template = std::fs::read_to_string("template.txt").unwrap();
        std::fs::write(format!("{}/jsonc.include.h", to), template).unwrap();
    }

    // TODO: generate code
}

use std::collections::HashSet;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DSL {
    pub types: Vec<Type>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum Type {
    Struct(Struct),
    Union(Union),
    Enum(Enum),
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Union {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<String>,
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

    fn validate_field(dsl: &DSL, field: &Field) {
        validate_type_name(&field.name);
        validate_name(&field.ty);
        if !is_builtin_type(&field.ty)
            && !dsl.types.iter().any(|ty| match ty {
                Type::Struct(s) => s.name == field.ty,
                Type::Union(u) => u.name == field.ty,
                Type::Enum(e) => e.name == field.ty,
            })
        {
            panic!("Unknown type: {}", field.ty);
        }
    }

    fn validate_type(dsl: &DSL, ty: &Type) {
        match ty {
            Type::Struct(s) => {
                validate_type_name(&s.name);
                s.fields.iter().for_each(|f| validate_field(dsl, f));
            }
            Type::Union(u) => {
                validate_type_name(&u.name);
                u.variants.iter().for_each(|n| {
                    validate_name(n);
                    if !is_builtin_type(n)
                        && !dsl.types.iter().any(|ty| match ty {
                            Type::Struct(s) => s.name == *n,
                            Type::Union(u) => u.name == *n,
                            Type::Enum(e) => e.name == *n,
                        })
                    {
                        panic!("Unknown type: {}", n);
                    }
                });
            }
            Type::Enum(e) => {
                validate_type_name(&e.name);
                e.variants.iter().for_each(|n| validate_type_name(n));
            }
        }
    }

    if dsl
        .types
        .iter()
        .map(|ty| match ty {
            Type::Struct(s) => s.name.to_string(),
            Type::Union(u) => u.name.to_string(),
            Type::Enum(e) => e.name.to_string(),
        })
        .collect::<HashSet<_>>()
        .len()
        != dsl.types.len()
    {
        panic!("Duplicate type name");
    }
    dsl.types.iter().for_each(|ty| validate_type(dsl, ty));
}

pub fn generate_code(dsl: &DSL, to: &str, c: bool, cpp: bool, json_schema: bool) {
    validate(dsl);

    // TODO: generate code
}

#[derive(Debug, Clone)]
pub enum Value {
    Str,
    Int,
    Long,
    Bool,
}

#[derive(Debug, Clone)]
pub enum InnerValue {
    Str(String),
    Int(i32),
    Long(i64),
    Bool(bool),
    None,
}
#[allow(private_in_public)]
impl From<(String, &Value)> for InnerValue {
    fn from(value: (String, &Value)) -> InnerValue {
        match value.1 {
            Value::Bool => match value.0.to_ascii_lowercase().as_ref() {
                "true" => InnerValue::Bool(true),
                "false" => InnerValue::Bool(false),
                _ => InnerValue::None,
            },
            Value::Int => {
                let parsed = value.0.parse();
                if parsed.is_err() {
                    return InnerValue::None;
                }
                InnerValue::Int(parsed.unwrap())
            }
            Value::Long => {
                let parsed = value.0.parse();
                if parsed.is_err() {
                    return InnerValue::None;
                }
                InnerValue::Long(parsed.unwrap())
            }
            Value::Str => InnerValue::Str(value.0),
        }
    }
}

impl From<InnerValue> for Option<i64> {
    fn from(value: InnerValue) -> Self {
        match value {
            InnerValue::Long(val) => Some(val),
            _ => None,
        }
    }
}

impl From<InnerValue> for Option<i32> {
    fn from(value: InnerValue) -> Self {
        match value {
            InnerValue::Int(num) => Some(num),
            _ => None,
        }
    }
}
impl From<InnerValue> for Option<String> {
    fn from(value: InnerValue) -> Self {
        match value {
            InnerValue::Str(str) => Some(str),
            _ => None,
        }
    }
}
impl From<InnerValue> for Option<bool> {
    fn from(value: InnerValue) -> Self {
        match value {
            InnerValue::Bool(flag) => Some(flag),
            _ => None,
        }
    }
}

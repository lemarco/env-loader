use std::collections::HashMap;
use std::env;
#[derive(Debug, Clone)]
pub enum Value {
    Str,
    Int,
    Long,
    Bool,
}
#[derive(Debug, Clone)]
enum InnerValue {
    Str(String),
    Int(i32),
    Long(i64),
    Bool(bool),
    None,
}

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
            // Value::None => InnerValue::None,
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

#[derive(Debug)]
pub enum ConfigLoaderError {
    NoEnvFile,
    ValueNotInEnv(&'static str),
    IncorrectValueType(&'static str),
    IsNotPartOfRuntime(&'static str),
    WrongConvertion,
    WrongTypeTryingToGet(&'static str),
}

pub struct ConfigLoader(HashMap<String, InnerValue>);

impl ConfigLoader {
    pub fn new(names: &[(&'static str, Value)]) -> Result<Self, ConfigLoaderError> {
        let dotenv_reading_result = dotenv::dotenv();
        if dotenv_reading_result.is_err() {
            return Err(ConfigLoaderError::NoEnvFile);
        }
        dotenv_reading_result.ok();
        let mut store = HashMap::new();
        for (name, typing) in names {
            let value = env::var(name);
            if value.is_err() {
                return Err(ConfigLoaderError::ValueNotInEnv(name));
            }
            let res: InnerValue = (value.unwrap(), typing).into();
            match res {
                InnerValue::None => return Err(ConfigLoaderError::WrongConvertion),
                val => store.insert(String::from(*name), val),
            };
        }
        Ok(Self(store))
    }

    pub fn get<T>(&self, name: &'static str) -> Result<T, ConfigLoaderError>
    where
        Option<T>: From<InnerValue>,
    {
        let val = self.0.get(name);
        if val.is_none() {
            return Err(ConfigLoaderError::IsNotPartOfRuntime(name));
        }

        match (*val.unwrap()).clone().into() {
            Some(inner) => Ok(inner),
            None => Err(ConfigLoaderError::WrongTypeTryingToGet(name)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_int() {
        let store = ConfigLoader::new(&[("PORT", Value::Int)]).unwrap();
        let port: i32 = store.get("PORT").unwrap();
        assert_eq!(port, 9999);
    }
    #[test]
    fn check_str() {
        let store = ConfigLoader::new(&[("HOST", Value::Str)]).unwrap();
        let host: String = store.get("HOST").unwrap();

        assert_eq!(host, "localhost");
    }
    #[test]
    fn check_bool() {
        let store = ConfigLoader::new(&[("CRITICAL_FLAG", Value::Bool)]).unwrap();
        let flag: bool = store.get("CRITICAL_FLAG").unwrap();

        assert!(flag);
    }
    #[test]
    fn check_long() {
        let store = ConfigLoader::new(&[("LONG_VAR", Value::Long)]).unwrap();
        let num: i64 = store.get("LONG_VAR").unwrap();

        assert_eq!(num, 5405632342349523);
    }
}

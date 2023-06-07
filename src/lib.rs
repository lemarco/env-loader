pub use env_loader_convert::convert;
mod error;
mod inner_value;
#[cfg(test)]
mod tests;
mod validators;
use error::{ConfigLoaderError, ConstraintValidationError};
use inner_value::{InnerValue, Value};
use std::collections::HashMap;
use std::env;
use validators::Constraint;

pub struct ConfigLoader(HashMap<String, InnerValue>);

fn parse_constraint(constraint_mask: &str) -> Option<Vec<Constraint>> {
    if constraint_mask == ",,," || constraint_mask.is_empty() {
        return None;
    }
    let splitted: Vec<&str> = constraint_mask.split(',').collect();
    let mut res = vec![];
    for (index, value) in splitted.iter().enumerate() {
        if splitted[index].is_empty() {
            continue;
        }
        match index {
            0 => res.push(Constraint::Max(value.parse().unwrap())),
            1 => res.push(Constraint::Min(value.parse().unwrap())),
            2 => res.push(Constraint::Optional),
            3 => res.push(Constraint::NotEmpty),
            _ => continue,
        }
    }
    Some(res)
}

fn check(
    val: &inner_value::InnerValue,
    constraints: &Option<Vec<Constraint>>,
) -> Result<bool, ConstraintValidationError> {
  
    match constraints {
        Some(constraints) => match val {
            InnerValue::Int(val) => validators::check_num(*val as i64, constraints),
            InnerValue::Long(val) => validators::check_num(*val, constraints),
            InnerValue::Str(val) => validators::check_str(val, constraints),
            _ => Ok(true),
        },
        None => Ok(true),
    }
}
impl ConfigLoader {
    pub fn new<T>(names: T) -> Result<Self, ConfigLoaderError>
    where
        T: IntoIterator<Item = (&'static str, Value, String)>,
    {
        let dotenv_reading_result = dotenv::dotenv();
        if dotenv_reading_result.is_err() {
            return Err(ConfigLoaderError::NoEnvFile);
        }

        let mut store = HashMap::new();
        for (name, typing, constraints) in names {
            let value = env::var(name);
            let constraints = parse_constraint(&constraints);
            let is_optional_value = constraints
                .as_ref()
                .is_some_and(|constraints| constraints.contains(&Constraint::Optional));
            if value.is_err() && !is_optional_value {
                return Err(ConfigLoaderError::ValueNotInEnv(format!(
                    "{} not in env file. Add it to file or mark as optional",
                    name
                )));
            }
            match value {
                Ok(value) => {
                    let res: InnerValue = (value, &typing).into();

                    match res {
                        InnerValue::None => return Err(ConfigLoaderError::WrongConvertion),
                        val => match check(&val, &constraints) {
                            Err(e) => {
                                return Err(ConfigLoaderError::ValueValidationFail(e));
                            }

                            Ok(_) => store.insert(String::from(name), val),
                        },
                    };
                }
                Err(_) => {
                    if !is_optional_value {
                        return Err(ConfigLoaderError::ValueNotInEnv(format!(
                            "{} not in env file. Add it to file or mark as optional",
                            name
                        )));
                    }
                }
            }
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

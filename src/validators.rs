use crate::error::ConstraintValidationError;
#[derive(Debug, PartialEq)]
pub enum Constraint {
    Min(i64),
    Max(i64),
    NotEmpty,
    Optional,
}

pub(crate) fn check_num(val: i64, cons: &[Constraint]) -> Result<bool, ConstraintValidationError> {
    for con in cons {
        match con {
            Constraint::Min(con_val) => {
                if *con_val > val {
                    return Err(ConstraintValidationError::MinConstraintViolation);
                }
            }
            Constraint::Max(con_val) => {
                if val > *con_val {
                    return Err(ConstraintValidationError::MaxConstraintViolation);
                }
            }
            _ => continue,
        }
    }
    Ok(true)
}
pub(crate) fn check_str(val: &str, cons: &[Constraint]) -> Result<bool, ConstraintValidationError> {
    for con in cons {
        match con {
            Constraint::Min(con_val) => {
                if *con_val > val.len() as i64 {
                    return Err(ConstraintValidationError::MinConstraintViolation);
                }
            }
            Constraint::Max(con_val) => {
                if val.len() as i64 > *con_val {
                    return Err(ConstraintValidationError::MaxConstraintViolation);
                }
            }
            Constraint::NotEmpty => {
                if val.is_empty() {
                    return Err(ConstraintValidationError::NotEmptyConstraintViolation);
                }
            }
            _ => continue,
        }
    }
    Ok(true)
}

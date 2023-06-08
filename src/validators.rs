use crate::error::ConstraintValidationError;
#[derive(Debug, PartialEq)]
pub enum Constraint {
    Min(i64),
    Max(i64),
    NotEmpty,
    Optional,
    Len(usize),
    // MinMax(i64, i64),
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
            // Constraint::MinMax(min, max) => {
            //     if val > *max || val < *min {
            //         return Err(ConstraintValidationError::MaxConstraintViolation);
            //     }
            // }
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
            // Constraint::MinMax(min, max) => {
            //     let len = val.len() as i64;
            //     if len > *max || len < *min {
            //         return Err(ConstraintValidationError::MaxConstraintViolation);
            //     }
            // }
            Constraint::Len(len) => {
                if *len != val.len() {
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

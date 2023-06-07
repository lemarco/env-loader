#[derive(Debug)]
pub enum ConfigLoaderError {
    NoEnvFile,
    ValueNotInEnv(String),
    IncorrectValueType(&'static str),
    IsNotPartOfRuntime(&'static str),
    WrongConvertion,
    WrongTypeTryingToGet(&'static str),
    ValueValidationFail(ConstraintValidationError),
}
#[derive(Debug)]
pub enum ConstraintValidationError {
    MinConstraintViolation,
    MaxConstraintViolation,
    NotEmptyConstraintViolation,
}

pub trait Validatable {
    fn validate(&self) -> Result<(), String>;
}

pub trait Constraint {
    fn name(&self) -> &'static str;
    fn validate(&self, _text: &str) -> bool {
        true
    }
}

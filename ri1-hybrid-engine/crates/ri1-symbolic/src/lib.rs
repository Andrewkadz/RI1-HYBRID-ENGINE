 use ri1_core::constraints::{Constraint, ConstraintEngine, ConstraintResult, Severity};

 pub struct NonEmpty;

 impl Constraint for NonEmpty {
     fn name(&self) -> &'static str { "non_empty" }
     fn check(&self, text: &str) -> ConstraintResult {
         let passed = !text.trim().is_empty();
         ConstraintResult {
             passed,
             severity: Severity::Hard,
             name: self.name(),
             message: if passed { None } else { Some("content must not be empty".into()) },
         }
     }
 }

 pub struct MaxLength(pub usize);

 impl Constraint for MaxLength {
     fn name(&self) -> &'static str { "max_length" }
     fn check(&self, text: &str) -> ConstraintResult {
         let passed = text.len() <= self.0;
         ConstraintResult {
             passed,
             severity: Severity::Soft,
             name: self.name(),
             message: if passed { None } else { Some(format!("length {} exceeds {}", text.len(), self.0)) },
         }
     }
 }

pub struct SymbolicEngine {
    rules: Vec<Box<dyn Constraint + Send + Sync>>,
}

impl SymbolicEngine {
    pub fn new(rules: Vec<Box<dyn Constraint + Send + Sync>>) -> Self { Self { rules } }
    pub fn new_default() -> Self {
        Self::new(vec![Box::new(NonEmpty), Box::new(MaxLength(280))])
    }
}

impl ConstraintEngine for SymbolicEngine {
    fn evaluate(&self, _modality: &str, content: &str) -> Vec<ConstraintResult> {
        self.rules.iter().map(|r| r.check(content)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn non_empty_works() {
        let r = NonEmpty;
        assert!(!r.check("").passed);
        assert!(r.check("ok").passed);
    }
}

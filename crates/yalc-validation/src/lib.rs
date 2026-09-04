pub mod extractor;

pub use extractor::ValidatedJson;
pub use validator::{Validate, ValidationError, ValidationErrors};

pub fn setup() {
    println!("yalc-validation initialized: AutoZod validation extractor is ready.");
}

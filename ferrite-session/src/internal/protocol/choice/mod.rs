pub mod either;

mod extract;
mod external_choice;
mod internal_choice;

pub use extract::{extract, ExtractChoice};
pub use external_choice::ExternalChoice;
pub use internal_choice::InternalChoice;

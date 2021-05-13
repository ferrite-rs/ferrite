pub mod either;

mod external_choice;
mod internal_choice;

pub use external_choice::{
  ExternalChoice,
  ExternalChoiceX,
};
pub use internal_choice::InternalChoice;

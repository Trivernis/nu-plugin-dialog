use std::io;

use nu_plugin::LabeledError;
mod generic_select;
pub use generic_select::GenericSelect;

pub trait UserPrompt {
    type Output;

    fn prompt(&self) -> Result<Self::Output, LabeledError>;

    fn prompt_opt(&self) -> Result<Option<Self::Output>, LabeledError>;
}

impl<'a> UserPrompt for dialoguer::Confirm<'a> {
    type Output = bool;

    fn prompt(&self) -> Result<Self::Output, LabeledError> {
        self.interact().map_err(create_labeled_error)
    }

    fn prompt_opt(&self) -> Result<Option<Self::Output>, LabeledError> {
        self.interact_opt().map_err(create_labeled_error)
    }
}

fn create_labeled_error(e: io::Error) -> LabeledError {
    LabeledError {
        label: "Failed to prompt user".into(),
        msg: e.to_string(),
        span: None,
    }
}

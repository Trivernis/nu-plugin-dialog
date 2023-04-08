use std::io;

use dialoguer::Password;
use nu_plugin::LabeledError;
mod generic_select;
pub use generic_select::GenericSelect;
use nu_protocol::Span;

pub trait UserPrompt {
    type Output;

    fn ask(&self, span: Span) -> Result<Self::Output, LabeledError>;

    fn ask_opt(&self, span: Span) -> Result<Option<Self::Output>, LabeledError>;
}

impl<'a> UserPrompt for dialoguer::Confirm<'a> {
    type Output = bool;

    fn ask(&self, span: Span) -> Result<Self::Output, LabeledError> {
        self.interact().map_err(|e| create_labeled_error(e, span))
    }

    fn ask_opt(&self, span: Span) -> Result<Option<Self::Output>, LabeledError> {
        self.interact_opt()
            .map_err(|e| create_labeled_error(e, span))
    }
}

impl<'a> UserPrompt for Password<'a> {
    type Output = String;

    fn ask(&self, span: Span) -> Result<Self::Output, LabeledError> {
        self.interact().map_err(|e| create_labeled_error(e, span))
    }

    fn ask_opt(&self, span: Span) -> Result<Option<Self::Output>, LabeledError> {
        self.ask(span).map(Option::Some)
    }
}

fn create_labeled_error(e: io::Error, span: Span) -> LabeledError {
    LabeledError {
        label: "Failed to prompt user".into(),
        msg: e.to_string(),
        span: Some(span),
    }
}

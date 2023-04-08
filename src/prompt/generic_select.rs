use dialoguer::{theme::Theme, FuzzySelect, Select};
use nu_protocol::Span;

use super::{create_labeled_error, UserPrompt};

pub enum GenericSelect<'a> {
    Fuzzy(FuzzySelect<'a>),
    Normal(Select<'a>),
}

impl<'a> GenericSelect<'a> {
    pub fn fuzzy(theme: &'a dyn Theme) -> Self {
        Self::Fuzzy(FuzzySelect::with_theme(theme))
    }

    pub fn normal(theme: &'a dyn Theme) -> Self {
        Self::Normal(Select::with_theme(theme))
    }

    pub fn items<T: ToString>(&mut self, items: &[T]) -> &mut Self {
        match self {
            GenericSelect::Fuzzy(f) => f.items(items).nop(),
            GenericSelect::Normal(n) => n.items(items).nop(),
        }
        self
    }

    pub fn default(&mut self, val: usize) -> &mut Self {
        match self {
            GenericSelect::Fuzzy(f) => f.default(val).nop(),
            GenericSelect::Normal(n) => n.default(val).nop(),
        }
        self
    }

    pub fn with_prompt<S: Into<String>>(&mut self, prompt: S) -> &mut Self {
        match self {
            GenericSelect::Fuzzy(f) => f.with_prompt(prompt).nop(),
            GenericSelect::Normal(n) => n.with_prompt(prompt).nop(),
        }
        self
    }
}

impl<'a> UserPrompt for GenericSelect<'a> {
    type Output = usize;

    fn ask(&self, span: Span) -> Result<Self::Output, nu_plugin::LabeledError> {
        match self {
            GenericSelect::Fuzzy(f) => f.interact(),
            GenericSelect::Normal(n) => n.interact(),
        }
        .map_err(|e| create_labeled_error(e, span))
    }

    fn ask_opt(&self, span: Span) -> Result<Option<Self::Output>, nu_plugin::LabeledError> {
        match self {
            GenericSelect::Fuzzy(f) => f.interact_opt(),
            GenericSelect::Normal(n) => n.interact_opt(),
        }
        .map_err(|e| create_labeled_error(e, span))
    }
}

trait Nop {
    fn nop(&mut self) {}
}

impl<'a> Nop for Select<'a> {}
impl<'a> Nop for FuzzySelect<'a> {}

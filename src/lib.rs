use dialoguer::theme::{ColorfulTheme, Theme};
use nu_plugin::{LabeledError, Plugin};
use nu_protocol::{PluginSignature, SyntaxShape};

mod confirm;
mod password;
mod prompt;
mod select;

pub struct DialogPlugin {
    pub(crate) theme: Box<dyn Theme>,
}

impl DialogPlugin {
    pub fn new() -> Self {
        Self {
            theme: Box::new(ColorfulTheme::default()),
        }
    }
}

impl Plugin for DialogPlugin {
    fn signature(&self) -> Vec<nu_protocol::PluginSignature> {
        vec![
            PluginSignature::build("ask").required(
                "subcommand",
                SyntaxShape::String,
                "The ask subcommand to run",
            ),
            PluginSignature::build("ask confirm")
                .usage("Prompt the user with a confirmation prompt.")
                .required(
                    "prompt",
                    SyntaxShape::String,
                    "The question to ask the user.",
                )
                .switch("abortable", "If set users can abort the prompt.", None)
                .named(
                    "default",
                    SyntaxShape::Boolean,
                    "The default selection.",
                    None,
                )
                .category(nu_protocol::Category::Misc),
            PluginSignature::build("ask select")
                .usage("Prompt the user with a selection prompt.")
                .required(
                    "items",
                    SyntaxShape::List(Box::new(SyntaxShape::String)),
                    "The items out of which one can be selected.",
                )
                .switch("fuzzy", "To add a fuzzy search to the select.", None)
                .switch("abortable", "If set users can abort the prompt.", None)
                .named(
                    "prompt",
                    SyntaxShape::String,
                    "An optional prompt that can be shown to the user for the selection.",
                    None,
                )
                .named(
                    "default",
                    SyntaxShape::Number,
                    "The default selection.",
                    None,
                )
                .category(nu_protocol::Category::Misc),
            PluginSignature::build("ask password")
                .usage("Prompt the user with a password input.")
                .named(
                    "prompt",
                    SyntaxShape::String,
                    "The prompt to this password input",
                    None,
                )
                .switch(
                    "confirm",
                    "Prompts the user twice for matching password inputs",
                    None,
                )
                .switch(
                    "allow-empty",
                    "Allows the user to input an empty password",
                    None,
                )
                .category(nu_protocol::Category::Misc),
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &nu_plugin::EvaluatedCall,
        input: &nu_protocol::Value,
    ) -> Result<nu_protocol::Value, nu_plugin::LabeledError> {
        match name {
            "ask confirm" => self.confirm(call, input),
            "ask select" => self.select(call, input),
            "ask password" => self.password(call, input),
            "ask" => 
            Err(LabeledError {
                label: "Missing subcommand".into(),
                msg: "the subcommand to the ask command is missing".into(),
                span: Some(call.head),
            }),
            _ =>  Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            })
        }
    }
}

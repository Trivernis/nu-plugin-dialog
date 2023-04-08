use dialoguer::theme::{ColorfulTheme, Theme};
use nu_plugin::{LabeledError, Plugin};
use nu_protocol::{PluginSignature, SyntaxShape};

mod confirm;

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
                .named(
                    "default",
                    SyntaxShape::Boolean,
                    "The default selection.",
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

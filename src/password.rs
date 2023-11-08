use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::Value;

use crate::{prompt::UserPrompt, DialogPlugin};

impl DialogPlugin {
    pub(crate) fn password(
        &self,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let mut pw_input = dialoguer::Password::with_theme(&*self.theme);

        pw_input.allow_empty_password(call.has_flag("allow-empty"));

        if call.has_flag("confirm") {
            pw_input.with_confirmation("Repeat your input", "Error: The inputs don't match");
        }

        let password = pw_input.ask(call.head)?;

        Ok(Value::string(password, call.head))
    }
}

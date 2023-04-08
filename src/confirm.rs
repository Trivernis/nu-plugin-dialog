use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::Value;

use crate::{prompt::UserPrompt, DialogPlugin};

impl DialogPlugin {
    pub(crate) fn confirm(
        &self,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let prompt: String = call.req(0)?;
        let default_val: Option<bool> = call.get_flag("default")?;

        let mut confirm = dialoguer::Confirm::with_theme(&*self.theme);
        confirm.with_prompt(prompt);

        if let Some(val) = default_val {
            confirm.default(val);
        }

        if call.has_flag("abortable") {
            let result = confirm.ask_opt(call.head)?;

            if let Some(val) = result {
                Ok(Value::Bool {
                    val,
                    span: call.head,
                })
            } else {
                Ok(Value::Nothing { span: call.head })
            }
        } else {
            let result = confirm.ask(call.head)?;
            Ok(Value::Bool {
                val: result,
                span: call.head,
            })
        }
    }
}

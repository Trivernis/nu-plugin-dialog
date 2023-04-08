use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::Value;

use crate::{
    prompt::{GenericSelect, UserPrompt},
    DialogPlugin,
};

impl DialogPlugin {
    pub(crate) fn select(
        &self,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let mut options: Vec<String> = call.req(0)?;

        let mut select = if call.has_flag("fuzzy") {
            GenericSelect::fuzzy(&*self.theme)
        } else {
            GenericSelect::normal(&*self.theme)
        };
        select.items(&options);

        if let Some(prompt) = call.get_flag::<String>("prompt")? {
            select.with_prompt(prompt);
        }
        if let Some(def) = call.get_flag::<usize>("default")? {
            select.default(def);
        }

        let selection = select.prompt()?;
        let selected_item = options.remove(selection);

        Ok(Value::String {
            val: selected_item,
            span: call.head,
        })
    }
}

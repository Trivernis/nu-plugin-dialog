use dialoguer::MultiSelect;
use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{Span, Value};

use crate::{prompt::UserPrompt, DialogPlugin};

impl DialogPlugin {
    pub(crate) fn multiselect(
        &self,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let options: Vec<String> = call.req(0)?;

        let mut select = MultiSelect::new();
        select.items(&options);

        if let Some(prompt) = call.get_flag::<String>("prompt")? {
            select.with_prompt(prompt);
        }
        if let Some(def) = call.get_flag::<String>("default")? {
            let defaults = def
                .split(',')
                .map(|v| v.trim())
                .filter_map(|v| v.parse::<usize>().ok())
                .collect::<Vec<_>>();
            let check_states = (0..options.len())
                .map(|i| defaults.contains(&i))
                .collect::<Vec<_>>();
            select.defaults(&check_states);
        }

        if call.has_flag("abortable") {
            if let Some(selection) = select.ask_opt(call.head)? {
                Ok(map_selection(selection, options, call.head))
            } else {
                Ok(Value::nothing(call.head))
            }
        } else {
            let selection = select.ask(call.head)?;

            Ok(map_selection(selection, options, call.head))
        }
    }
}

fn map_selection(selection: Vec<usize>, options: Vec<String>, span: Span) -> Value {
    let selected_items = options
        .into_iter()
        .enumerate()
        .filter(|(i, _)| selection.contains(i))
        .map(|(_, val)| Value::string(val, span))
        .collect::<Vec<_>>();
    Value::list(selected_items, span)
}

use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_dialog::DialogPlugin;

fn main() {
    serve_plugin(&mut DialogPlugin::new(), MsgPackSerializer)
}

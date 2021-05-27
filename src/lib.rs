#[macro_use]
extern crate vst;

use vst::plugin::{HostCallback, Info, Plugin, Category};

struct VstTemplatePlugin;

impl Plugin for VstTemplatePlugin {
    fn new(_host: HostCallback) -> Self {
        VstTemplatePlugin
    }

    fn get_info(&self) -> Info {
        Info {
            name: "Vst Template Plugin".to_string(),
            vendor: "Oleg Patsay".to_string(),
            inputs: 2,
            outputs: 2,
            version: 0001,
            category: Category::Effect,
            unique_id: 1357, // Used by hosts to differentiate between plugins.

            ..Default::default()
        }
    }
}

plugin_main!(VstTemplatePlugin); // Important!
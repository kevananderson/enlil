use crate::data_dict::{DataTag, VarDictType as vdt};
use crate::module::{Module, ModuleCore};
use std::any::type_name;

fn inputs() -> Option<Vec<String>> {
    // let v = vec![
    //    "module>variable".to_string(),
    //];
    None
}

// declare the variables that this module outputs
fn data() -> Vec<DataTag> {
    vec![DataTag {
        key: "Temperature".to_string(),
        summary: "A Temperature (degrees C) read periodically.".to_string(),
        description: None,
        default: vdt::TempCentigrade(20.0),
    }]
}

fn state() -> Option<Vec<DataTag>> {
    // let v = vec![

    //     DataTag{
    //         key: "Value Key".to_string(),
    //         summary: "Short Description".to_string(),
    //         description: "Long Description".to_string(),
    //         default: vdt::TempCentigrade(0.0),
    //     },

    // ];
    // Some(v)
    None
}

#[derive(Default, Debug)]
pub struct TickModule {
    m_core: ModuleCore,
}

impl TickModule {
    pub fn new() -> Self {
        // create a new copy of this module
        let mut new_mod = TickModule {
            ..Default::default()
        };

        // setup the module core
        new_mod.m_core.setup_name(type_name::<TickModule>());
        new_mod.m_core.setup_output(data());
        new_mod.m_core.period = 1000;

        new_mod
    }
}

impl Module for TickModule {
    fn run(&self) {
        todo!();
    }

    fn core_mut(&mut self) -> &mut ModuleCore {
        &mut self.m_core
    }
    fn core(&self) -> &ModuleCore {
        &self.m_core
    }
}

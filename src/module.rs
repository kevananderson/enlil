use core::fmt;
use std::any::type_name;
use std::collections::HashMap;

use crate::data_dict::{strip_name, DataTag, VarDictType};

#[derive(Clone, Debug, Default)]
pub struct ModuleCore {
    name: String,
    path: String,
    pub period: i32, // period in ms
    input: HashMap<String, VarDictType>,
    state: HashMap<String, VarDictType>,
    output: HashMap<String, VarDictType>,
    data: Vec<DataTag>,
}

impl ModuleCore {
    pub fn setup_name(&mut self, full_name: &str) {
        self.name = strip_name(full_name);
        self.path = full_name.to_string();
    }

    pub fn setup_input(&mut self, input_list: &[String]) {
        todo!();
    }

    fn setup_state(&mut self, state_list: &Vec<DataTag>) {
        let mut map: HashMap<String, VarDictType> = HashMap::new();
        for tag in state_list {
            map.insert(tag.key.clone(), tag.default.clone());
        }
        self.state = map;
    }

    pub fn setup_output(&mut self, data_list: Vec<DataTag>) {
        self.data = data_list.clone();
        let mut map: HashMap<String, VarDictType> = HashMap::new();
        for tag in data_list {
            map.insert(tag.key.clone(), tag.default.clone());
        }
        self.output = map;
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for ModuleCore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub trait Module {
    // interface required to be implemented
    fn run(&self);
    fn core_mut(&mut self) -> &mut ModuleCore;
    fn core(&self) -> &ModuleCore;

    // trait default functions
}

// sugar
impl std::fmt::Debug for dyn Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(type_name::<Self>()).finish()
    }
}

use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufWriter, Write};

use crate::data_dict::strip_name;
use crate::module::Module;

#[derive(Default, Debug)]
pub struct ServiceCore {
    name: String,
    path: String,
    modules: Vec<Box<dyn Module>>,
}

impl ServiceCore {
    pub fn setup_name(&mut self, full_name: &str) {
        self.name = strip_name(full_name);
        self.path = full_name.to_string();
    }

    pub fn add_module(&mut self, module: Box<dyn Module>) {
        self.modules.push(module);
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub trait Service {
    // interface required to be implemented
    fn setup(&mut self);
    fn run(&mut self);
    fn core_mut(&mut self) -> &mut ServiceCore;
    fn core(&self) -> &ServiceCore;

    // trait default functions

    fn document(&self) -> io::Result<()> {
        let file = File::create("output.txt")?;
        let mut f = BufWriter::new(file);
        writeln!(f, "Service: {}", &self.core().name())?;
        writeln!(f, "Modules:")?;
        for module in &self.core().modules {
            writeln!(f, " - {}", module.core().name())?;
        }
        Ok(())
    }
}

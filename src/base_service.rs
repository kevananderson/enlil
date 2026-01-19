use std::any::type_name;

use crate::service::{Service, ServiceCore};
use crate::tick_module::TickModule;

#[derive(Default, Debug)]
pub struct BaseService {
    pub s_core: ServiceCore,
}

impl BaseService {
    pub fn new() -> Self {
        // create a new copy of this struct
        let mut svc = BaseService {
            ..Default::default()
        };

        // setup the service core
        svc.s_core.setup_name(type_name::<BaseService>());

        // add the modules
        svc.s_core.add_module(Box::new(TickModule::new()));

        svc
    }
}
impl Service for BaseService {

    fn setup(&mut self) {
        todo!();
    }

    fn run(&mut self) {
        todo!();
    }

    fn core_mut(&mut self) -> &mut ServiceCore {
        &mut self.s_core
    }
    fn core(&self) -> &ServiceCore {
        &self.s_core
    }
}

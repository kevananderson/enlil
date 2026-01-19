mod base_service;
mod service;

mod module;
mod tick_module;

mod data_dict;

use base_service::BaseService;

use crate::service::Service;

fn main() {
    let svc = BaseService::new();
    let _ = svc.document();
}

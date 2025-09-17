#![no_std]

use sails_rs::{
    prelude::*,
    cell::RefCell,
};

pub mod services;
pub mod state;

use services::contract_service::ContractService;
use state::StakingBroker;

pub struct ContractProgram {
    state: RefCell<StakingBroker>,
}

// Program contains "payable" argument because it will receive tokens
#[program(payable)]
impl ContractProgram {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(Default::default()),
        }
    }

    #[export(route = "ContractService")]
    pub fn contract_svc(&self) -> ContractService<'_> {
        ContractService::new(&self.state)
    }
}


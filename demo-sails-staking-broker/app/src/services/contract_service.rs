use sails_rs::{
    prelude::*,
    cell::RefCell,
};
use gbuiltin_staking::*;
use gstd::ext;

use crate::state::StakingBroker;


#[event]
#[derive(Encode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ContractEvent {
    BondedValue(u128),
    BondedExtraValue(u128),
    UnbondedValue(u128),
    WithdrawUnbonded,
    Nominated(Vec<ActorId>),
    Chill,
    PayoutStakers {
        validator: ActorId,
        era: u32
    },
    RebondedValue(u128),
    PayeeSet(RewardAccount)
}

#[derive(Encode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ContractResponse {
    BondedValue(u128),
    BondedExtraValue(u128),
    UnbondedValue(u128),
    WithdrawUnbonded,
    Nominated(Vec<ActorId>),
    Chill,
    PayoutStakers {
        validator: ActorId,
        era: u32
    },
    RebondedValue(u128),
    PayeeSet(RewardAccount)
}

pub struct ContractService<'a> {
    state: &'a RefCell<StakingBroker>,
}

impl <'a> ContractService<'a> {
    pub fn new(state: &'a RefCell<StakingBroker>) -> Self {
        Self { state }
    }
}

#[service(events = ContractEvent)]
impl ContractService<'_> {
    #[export]
    pub async fn bond(&mut self, value: u128, payee: RewardAccount) -> ContractResponse {
        let user_value = Syscall::message_value();

        if value != user_value {
            ("given value and tokens get must be equal!");
        }

        StakingBroker::bond(
            self.state.borrow_mut(), 
            value, 
            Some(payee)
        ).await;

        self.emit_event(ContractEvent::BondedValue(value)).unwrap();

        ContractResponse::BondedValue(value)
    }
 
    #[export]
    pub async fn bond_extra(&mut self, value: u128) -> ContractResponse {
        let user_value = Syscall::message_value();

        if value != user_value {
            ext::panic("given value and tokens get must be equal!");
        }

        StakingBroker::bond(
            self.state.borrow_mut(), 
            value, 
            None
        ).await;

        self.emit_event(ContractEvent::BondedExtraValue(value)).unwrap();

        ContractResponse::BondedExtraValue(value)
    }

    #[export]
    pub async fn unbond(&mut self, value: u128) -> ContractResponse {
        StakingBroker::unbond(
            self.state.borrow_mut(), 
            value
        ).await;

        self.emit_event(ContractEvent::UnbondedValue(value)).unwrap();
        
        ContractResponse::UnbondedValue(value)
    }

    #[export]
    pub async fn withdraw_unbonded(&mut self) -> ContractResponse {
        // [TODO]: checar esta parte

        StakingBroker::withdraw_unbonded(
            self.state.borrow()
        ).await;
        
        self.emit_event(ContractEvent::WithdrawUnbonded).unwrap();

        ContractResponse::WithdrawUnbonded
    }

    #[export]
    pub async fn nominate(&mut self, targets: Vec<ActorId>) -> ContractResponse {
        StakingBroker::nominate(
            self.state.borrow(), 
            targets.clone()
        ).await;

        self.emit_event(ContractEvent::Nominated(targets.clone())).unwrap();

        ContractResponse::Nominated(targets)
    }

    #[export]
    pub async fn chill(&mut self) -> ContractResponse {
        StakingBroker::chill(self.state.borrow()).await;

        self.emit_event(ContractEvent::Chill).unwrap();

        ContractResponse::Chill
    }

    #[export]
    pub async fn payout_stakers(&mut self, validator: ActorId, era: u32) -> ContractResponse {
        StakingBroker::payout_stakers(
            self.state.borrow(), 
            validator, 
            era
        ).await;

        self.emit_event(ContractEvent::PayoutStakers { validator, era }).unwrap();

        ContractResponse::PayoutStakers { 
            validator, 
            era 
        }
    }

    #[export]
    pub async fn rebond(&mut self, value: u128) -> ContractResponse {
        StakingBroker::rebond(
            self.state.borrow_mut(), 
            value
        ).await;

        self.emit_event(ContractEvent::RebondedValue(value)).unwrap();

        ContractResponse::RebondedValue(value)
    }

    #[export]
    pub async fn set_payee(&mut self, payee: RewardAccount) -> ContractResponse {
        StakingBroker::set_payee(
            self.state.borrow_mut(), 
            payee
        ).await;

        self.emit_event(ContractEvent::PayeeSet(payee)).unwrap();

        ContractResponse::PayeeSet(payee)
    }
}
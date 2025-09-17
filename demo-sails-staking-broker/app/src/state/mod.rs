use sails_rs::{
    prelude::*,
    collections::HashMap,
    gstd::{msg, debug},
    cell::{RefMut, Ref}
};
use gstd::{
    errors::Error, 
    actor_id
};
use gbuiltin_staking::*;

// Staking proxy builtin actor program id (hardcoded for all runtimes)
const BUILTIN_ADDRESS: ActorId = actor_id!("0x77f65ef190e11bfecb8fc8970fd3749e94bed66a23ec2f7a3623e785d0816761");

#[derive(Debug, Default)]
pub struct StakingBroker {
    /// Has bonded any amount yet
    has_bonded_any: bool,
    /// Total debit
    total_debit: u128,
    /// Registry of bonded deposits
    bonded: HashMap<ActorId, u128>,
    /// Reward payee account id
    reward_account: ActorId,
}

impl StakingBroker {
    /// Add bonded amount for the contract as both stash and controller.
    pub async fn bond(mut state: RefMut<'_, StakingBroker>, value: u128, payee: Option<RewardAccount>) {
        // Prepare a message to the built-in actor
        // Checking the flag to decide whether to use `Bond` or `BondExtra`
        // Note: this is not how you'd do it in a real application, given the
        // Staking pallet `unbonding` logic, but it's enough for the example.
        let payload = if !state.has_bonded_any {
            Request::Bond {
                value,
                payee: payee.unwrap_or(RewardAccount::Program),
            }
        } else {
            Request::BondExtra { value }
        };
        debug!(
            "[StakingBroker] Sending `bond` message {:?} at broker's state {:?}",
            payload, state
        );
        do_send_message(payload, || {
            // Update local state to account for value transfer in pallet
            state.bonded
                .entry(msg::source())
                .and_modify(|old| *old += value)
                .or_insert(value);
            state.total_debit += value;
            state.has_bonded_any = true;
            state.reward_account = match payee {
                Some(RewardAccount::Custom(account_id)) => account_id,
                _ => msg::source(),
            };
        })
        .await
    }

    pub async fn unbond(mut state: RefMut<'_, StakingBroker>, value: u128) {
        let source = msg::source();

        // The sender can unbond only so much as they have bonded
        let value = state.bonded.get(&source).map_or(0, |v| (*v).min(value));
        if value == 0 {
            debug!("[StakingBroker::unbond] No bonded amount");
            msg::reply_bytes(b"No bonded amount", 0).expect("Failed to send reply");
            return;
        }

        // Prepare a message to the built-in actor
        let payload = Request::Unbond { value };
        debug!(
            "[StakingBroker] Sending `unbond` message {:?} at broker's state {:?}",
            payload, state
        );
        do_send_message(payload, || {
            // Update local state
            if let Some(old) = state.bonded.get_mut(&source) {
                *old = old.saturating_sub(value);
            }
            state.total_debit = state.total_debit.saturating_sub(value);
        })
        .await
    }

    pub async fn nominate(state: Ref<'_, StakingBroker>, targets: Vec<ActorId>) {
        // Prepare a message to the built-in actor
        let payload = Request::Nominate { targets };
        debug!(
            "[StakingBroker] Sending `nominate` message {:?} at broker's state {:?}",
            payload, state
        );
        do_send_message(payload, || {}).await
    }

    pub async fn chill(state: Ref<'_, StakingBroker>) {
        // Prepare a message to the built-in actor
        let payload = Request::Chill {};
        debug!(
            "[StakingBroker] Sending `chill` message {:?} at broker's state {:?}",
            payload, state
        );
        do_send_message(payload, || {}).await
    }

    pub async fn rebond(mut state: RefMut<'_, StakingBroker>, value: u128) {
        let source = msg::source();

        // Prepare a message to the built-in actor
        let payload = Request::Rebond { value };
        debug!(
            "[StakingBroker] Sending `rebond` message {:?} at broker's state {:?}",
            payload, state
        );
        do_send_message(payload, || {
            // Update local state
            if let Some(old) = state.bonded.get_mut(&source) {
                *old = old.saturating_add(value);
            }
            state.total_debit = state.total_debit.saturating_add(value);
        })
        .await
    }

    pub async fn withdraw_unbonded(state: Ref<'_, StakingBroker>) {
        let _sender = msg::source();

        // Prepare a message to the built-in actor
        let payload = Request::WithdrawUnbonded {
            num_slashing_spans: 0,
        };
        debug!(
            "[StakingBroker] Sending `withdraw_unbonded` message {:?} at broker's state {:?}",
            payload, state
        );
        do_send_message(payload, || {
            // TODO: send a part of withdrawn amount to the sender and/or
            // some other users who requested unbonding earlier
        })
        .await
    }

    pub async fn set_payee(mut state: RefMut<'_, StakingBroker>, payee: RewardAccount) {
        // Prepare a message to the built-in actor
        let payload = Request::SetPayee { payee };
        debug!(
            "[StakingBroker] Sending `set_payee` message {:?} at broker's state {:?}",
            payload, state
        );
        do_send_message(payload, || {
            state.reward_account = match payee {
                RewardAccount::Custom(account_id) => account_id,
                _ => msg::source(),
            }
        })
        .await
    }

    pub async fn payout_stakers(state:  Ref<'_, StakingBroker>, validator_stash: ActorId, era: u32) {
        // Prepare a message to the built-in actor
        let payload = Request::PayoutStakers {
            validator_stash,
            era,
        };
        debug!(
            "[StakingBroker] Sending `payout_stakers` message {:?} at broker's state {:?}",
            payload, state
        );
        do_send_message(payload, || {
            // TODO: transfer fraction of rewards to nominators of the `validator_stash`
        })
        .await
    }
}

/// Do the actual message sending and reply handling.
async fn do_send_message<E: Encode>(payload: E, mut on_success: impl FnMut()) {
    match msg::send_for_reply(BUILTIN_ADDRESS, payload, 0, 0)
        .expect("Error sending message")
        .await
    {
        Ok(_) => {
            debug!("[StakingBroker] Success reply from builtin actor received");
            on_success();
            msg::reply_bytes(b"Success", 0).expect("Failed to send reply");
        }
        Err(e) => {
            debug!("[StakingBroker] Error reply from builtin actor received: {e:?}");
            match e {
                Error::ErrorReply(payload, _reason) => {
                    panic!("{}", payload);
                }
                _ => panic!("Error in upstream program"),
            }
        }
    };
}
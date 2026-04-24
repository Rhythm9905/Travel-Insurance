#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, symbol_short};

// Storage keys
#[contracttype]
pub enum DataKey {
    PolicyHolder(Address),
    Premium(Address),
    IsActive(Address),
    ClaimPaid(Address),
}

// Constants
const PAYOUT_AMOUNT: i128 = 1000; // Example fixed payout

#[contract]
pub struct TravelInsuranceContract;

#[contractimpl]
impl TravelInsuranceContract {

    // Buy insurance policy
    pub fn buy_policy(env: Env, user: Address, premium: i128) {
        user.require_auth();

        env.storage().instance().set(&DataKey::PolicyHolder(user.clone()), &true);
        env.storage().instance().set(&DataKey::Premium(user.clone()), &premium);
        env.storage().instance().set(&DataKey::IsActive(user.clone()), &true);
        env.storage().instance().set(&DataKey::ClaimPaid(user.clone()), &false);
    }

    // Check if policy is active
    pub fn is_active(env: Env, user: Address) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::IsActive(user))
            .unwrap_or(false)
    }

    // Trigger claim payout (e.g., flight delay verified off-chain)
    pub fn claim(env: Env, user: Address) {
        user.require_auth();

        let active: bool = env.storage()
            .instance()
            .get(&DataKey::IsActive(user.clone()))
            .unwrap_or(false);

        let already_paid: bool = env.storage()
            .instance()
            .get(&DataKey::ClaimPaid(user.clone()))
            .unwrap_or(false);

        if !active {
            panic!("Policy not active");
        }

        if already_paid {
            panic!("Claim already paid");
        }

        // Mark claim as paid
        env.storage().instance().set(&DataKey::ClaimPaid(user.clone()), &true);
        env.storage().instance().set(&DataKey::IsActive(user.clone()), &false);

        // Emit event (actual token transfer would require token contract integration)
        env.events().publish(
            (symbol_short!("CLAIM"), user),
            PAYOUT_AMOUNT
        );
    }

    // Check claim status
    pub fn claim_status(env: Env, user: Address) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::ClaimPaid(user))
            .unwrap_or(false)
    }
}
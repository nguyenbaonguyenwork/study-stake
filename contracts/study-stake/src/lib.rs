#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Symbol,
};

#[contracttype]
#[derive(Clone)]
pub struct Commitment {
    pub creator: Address,
    pub title: Symbol,
    pub stake: i128,
    pub completed: bool,
}

#[contracttype]
pub enum DataKey {
    Commitment(u32),
    Counter,
}

#[contract]
pub struct StudyStakeContract;

#[contractimpl]
impl StudyStakeContract {

    // CREATE COMMITMENT
    pub fn create_commitment(
        env: Env,
        creator: Address,
        title: Symbol,
        stake: i128,
    ) -> u32 {

        creator.require_auth();

        let mut counter: u32 = env
            .storage()
            .instance()
            .get(&DataKey::Counter)
            .unwrap_or(0);

        counter += 1;

        let commitment = Commitment {
            creator: creator.clone(),
            title,
            stake,
            completed: false,
        };

        env.storage()
            .instance()
            .set(&DataKey::Commitment(counter), &commitment);

        env.storage()
            .instance()
            .set(&DataKey::Counter, &counter);

        counter
    }

    // COMPLETE COMMITMENT
    pub fn complete_commitment(
        env: Env,
        id: u32,
    ) {

        let mut commitment: Commitment = env
            .storage()
            .instance()
            .get(&DataKey::Commitment(id))
            .unwrap();

        commitment.creator.require_auth();

        commitment.completed = true;

        env.storage()
            .instance()
            .set(&DataKey::Commitment(id), &commitment);
    }

    // GET COMMITMENT
    pub fn get_commitment(
        env: Env,
        id: u32,
    ) -> Commitment {

        env.storage()
            .instance()
            .get(&DataKey::Commitment(id))
            .unwrap()
    }
}

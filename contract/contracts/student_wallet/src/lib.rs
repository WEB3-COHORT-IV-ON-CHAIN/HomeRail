#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, Vec, Address, Symbol};

#[contract]
pub struct StudentWallet;

#[contractimpl]
impl StudentWallet {
    pub fn init(env: Env, student: Address, school: Address) -> Address {
        let contract_id = env.current_contract_address();

        let student_symbol = Symbol::new(&env, "school");
        let school_key = (student_symbol, &student);
        env.storage().persistent().set(&school_key, &school);

        let locked_symbol = Symbol::new(&env, "locked");
        let locked_key = (locked_symbol, &student);
        env.storage().persistent().set(&locked_key, &0i128);

        let free_symbol = Symbol::new(&env, "free");
        let free_key = (free_symbol, &student);
        env.storage().persistent().set(&free_key, &0i128);

        let deposited_symbol = Symbol::new(&env, "deposited");
        let deposited_key = (deposited_symbol, &student);
        env.storage().persistent().set(&deposited_key, &0i128);

        env.events().publish(
            (Symbol::new(&env, "StudentWallet"), Symbol::new(&env, "Initialized")),
            (student, school),
        );

        contract_id
    }

    pub fn register_school(env: Env, student: Address, school: Address) {
        let student_symbol = Symbol::new(&env, "school");
        let school_key = (student_symbol, &student);
        env.storage().persistent().set(&school_key, &school);

        env.events().publish(
            (Symbol::new(&env, "StudentWallet"), Symbol::new(&env, "SchoolRegistered")),
            (student, school),
        );
    }

    pub fn deposit(env: Env, student: Address, amount: i128) {
        let free_symbol = Symbol::new(&env, "free");
        let free_key = (free_symbol, &student);
        let mut free: i128 = env
            .storage()
            .persistent()
            .get(&free_key)
            .unwrap_or(0);
        free += amount;
        env.storage().persistent().set(&free_key, &free);

        let deposited_symbol = Symbol::new(&env, "deposited");
        let deposited_key = (deposited_symbol, &student);
        let mut deposited: i128 = env
            .storage()
            .persistent()
            .get(&deposited_key)
            .unwrap_or(0);
        deposited += amount;
        env.storage().persistent().set(&deposited_key, &deposited);

        env.events().publish(
            (Symbol::new(&env, "StudentWallet"), Symbol::new(&env, "Deposited")),
            (student, amount),
        );
    }

    pub fn release_to_school(env: Env, student: Address, amount: i128) {
        let locked_symbol = Symbol::new(&env, "locked");
        let locked_key = (locked_symbol, &student);
        let mut locked: i128 = env
            .storage()
            .persistent()
            .get(&locked_key)
            .unwrap_or(0);
        assert!(locked >= amount, "Insufficient locked funds");
        locked -= amount;
        env.storage().persistent().set(&locked_key, &locked);

        env.events().publish(
            (Symbol::new(&env, "StudentWallet"), Symbol::new(&env, "ReleasedToSchool")),
            (student, amount),
        );
    }

    pub fn release_free(env: Env, student: Address, amount: i128) {
        let free_symbol = Symbol::new(&env, "free");
        let free_key = (free_symbol, &student);
        let mut free: i128 = env
            .storage()
            .persistent()
            .get(&free_key)
            .unwrap_or(0);
        assert!(free >= amount, "Insufficient free funds");
        free -= amount;
        env.storage().persistent().set(&free_key, &free);

        env.events().publish(
            (Symbol::new(&env, "StudentWallet"), Symbol::new(&env, "ReleasedFree")),
            (student, amount),
        );
    }
}

mod test;
#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Address, String};

const TEST_STUDENT: &str = "GBDZ3J6QP5I5X7K7L5M4N3O2P1Q0R9S8T7U5";
const TEST_SCHOOL: &str = "SBDZ3J6QP5I5X7K7L5M4N3O2P1Q0R9S8T7U5";

#[test]
fn test_init() {
    let env = Env::default();
    let contract_id = env.register(StudentWallet, ());

    let student = Address::from_string(&String::from_str(&env, TEST_STUDENT));
    let school = Address::from_string(&String::from_str(&env, TEST_SCHOOL));
    let client = StudentWalletClient::new(&env, &contract_id);

    // init should not panic
    client.init(&student, &school);
}

#[test]
fn test_deposit_success() {
    let env = Env::default();
    let contract_id = env.register(StudentWallet, ());

    let student = Address::from_string(&String::from_str(&env, TEST_STUDENT));
    let school = Address::from_string(&String::from_str(&env, TEST_SCHOOL));
    let client = StudentWalletClient::new(&env, &contract_id);

    client.init(&student, &school);
    // deposit should not panic
    client.deposit(&student, &500i128);
}

#[test]
fn test_release_to_school_success() {
    let env = Env::default();
    let contract_id = env.register(StudentWallet, ());

    let student = Address::from_string(&String::from_str(&env, TEST_STUDENT));
    let school = Address::from_string(&String::from_str(&env, TEST_SCHOOL));
    let client = StudentWalletClient::new(&env, &contract_id);

    client.init(&student, &school);
    client.deposit(&student, &1000i128);
    // release_to_school should not panic
    client.release_to_school(&student, &800i128);
}

#[test]
fn test_release_free_success() {
    let env = Env::default();
    let contract_id = env.register(StudentWallet, ());

    let student = Address::from_string(&String::from_str(&env, TEST_STUDENT));
    let client = StudentWalletClient::new(&env, &contract_id);

    client.init(&student, &student);
    client.deposit(&student, &1000i128);
    // release_free should not panic
    client.release_free(&student, &600i128);
}
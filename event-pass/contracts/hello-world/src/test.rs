#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_event_pass() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(EventPass, ());
    let client = EventPassClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // 1. El usuario comienza sin pase.
    assert_eq!(client.has_pass(&user), false);

    // 2. Compra el pase.
    client.buy_pass(&user);

    // 3. Ahora tiene el pase.
    assert_eq!(client.has_pass(&user), true);

    // 4. Todavia no fue utilizado.
    assert_eq!(client.is_used(&user), false);

    // 5. Utiliza el pase.
    client.use_pass(&user);

    // 6. Queda marcado como utilizado.
    assert_eq!(client.is_used(&user), true);
}

#[test]
#[should_panic(expected = "Pass already used")]
fn test_no_se_puede_usar_dos_veces() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(EventPass, ());
    let client = EventPassClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    client.buy_pass(&user);
    client.use_pass(&user);
    // El segundo intento debe ser rechazado por el propio contrato.
    client.use_pass(&user);
}

#[test]
#[should_panic(expected = "No pass")]
fn test_sin_pase_no_puede_usar() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(EventPass, ());
    let client = EventPassClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    client.use_pass(&user);
}

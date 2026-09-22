#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

/// Claves de almacenamiento del contrato.
/// Para cada address guardamos dos valores independientes:
/// - Pass: si compro el pase
/// - Used: si ya lo utilizo
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Pass(Address),
    Used(Address),
}

#[contract]
pub struct EventPass;

#[contractimpl]
impl EventPass {
    /// Registra que una address compro su pase.
    /// Estado resultante: pass = true, used = false.
    pub fn buy_pass(env: Env, user: Address) {
        user.require_auth();

        env.storage()
            .persistent()
            .set(&DataKey::Pass(user.clone()), &true);
        env.storage()
            .persistent()
            .set(&DataKey::Used(user), &false);
    }

    /// Devuelve true si la address tiene un pase registrado.
    pub fn has_pass(env: Env, user: Address) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::Pass(user))
            .unwrap_or(false)
    }

    /// Devuelve true si el pase de la address ya fue utilizado.
    pub fn is_used(env: Env, user: Address) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::Used(user))
            .unwrap_or(false)
    }

    /// Utiliza el pase. Solo se puede usar una vez:
    /// falla si la address no tiene pase o si ya lo uso.
    pub fn use_pass(env: Env, user: Address) {
        user.require_auth();

        let has_pass: bool = env
            .storage()
            .persistent()
            .get(&DataKey::Pass(user.clone()))
            .unwrap_or(false);
        if !has_pass {
            panic!("No pass");
        }

        let used: bool = env
            .storage()
            .persistent()
            .get(&DataKey::Used(user.clone()))
            .unwrap_or(false);
        if used {
            panic!("Pass already used");
        }

        env.storage()
            .persistent()
            .set(&DataKey::Used(user), &true);
    }
}

mod test;

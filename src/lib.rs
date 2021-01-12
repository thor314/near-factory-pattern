#![allow(unused_imports)]
#![allow(dead_code)]
use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  collections::*,
  serde::{Deserialize, Serialize},
  serde_json::json,
  *,
};

mod utils;

#[cfg(target = "wasm32")]
#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
const GAS: Gas = 10_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StructInit {
  a: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Wrap {
  a: String,
}

/// Default must be implemented for wasm compilation.
impl Default for StructInit {
  fn default() -> Self {
    panic!("Struct_init")
  }
}

#[near_bindgen]
impl StructInit {
  #[init]
  pub fn new(wrap: Wrap) -> Self {
    assert!(!env::state_exists(), "Already, initialized");
    Self { a: wrap.a }
  }

  #[payable]
  pub fn deploy_subaccount1(&mut self, wrap: Wrap, sub: String) {
    let to = format!("{}.{}", sub, env::current_account_id());

    // method 1. try to serialize with serde.
    let args = serde_json::to_vec(&json!({"wrap": {"a": wrap.a}, "sub": sub}))
      .expect("failed to jsonify wrap");

    Promise::new(to)
      .create_account()
      .transfer(env::attached_deposit())
      .deploy_contract(include_bytes!("../res/struct_init.wasm").to_vec())
      .function_call(b"new".to_vec(), args, 0, GAS);
  }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
  fn test() {
    assert!(true);
  }
}

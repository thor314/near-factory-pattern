use crate::*;

pub fn log_stuff() {
  env::log(format!("prep gas: {}", env::prepaid_gas()).as_bytes());
  env::log(format!("current_account_id: {}", env::current_account_id()).as_bytes());
  env::log(format!("signer_account_id: {}", env::signer_account_id()).as_bytes());
  env::log(format!("predecessor_account_id: {}", env::predecessor_account_id()).as_bytes());
}

use alloy::primitives::Address;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub address: Address,
    pub account_type: AccountType,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    LocalSigner,
    HardwareWallet,
    ReadOnly,
}

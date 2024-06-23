use cosmwasm_std::Uint128;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {
    CreateToken { denom: String, name: String, symbol: String, decimals: u8 },
    Mint { denom: String, amount: Uint128 },
    Burn { denom: String, amount: Uint128 },
    Transfer { recipient: String, denom: String, amount: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QueryMsg {
    TokenInfo { denom: String },
    Balance { owner: String, denom: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TokenInfoResponse {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BalanceResponse {
    pub balance: Uint128,
}

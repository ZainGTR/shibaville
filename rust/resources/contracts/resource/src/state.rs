use cosmwasm_std::{Addr, Storage, Uint128, StdResult, StdError};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
}

pub const TOKENS: Map<&str, TokenInfo> = Map::new("tokens");
pub const BALANCES: Map<(&Addr, &str), Uint128> = Map::new("balances");

pub fn increase_balance(
    storage: &mut dyn Storage,
    owner: &Addr,
    denom: &str,
    amount: Uint128,
) -> StdResult<()> {
    let key = (owner, denom);
    let balance = BALANCES.may_load(storage, key)?.unwrap_or_default();
    BALANCES.save(storage, key, &(balance + amount))?;
    Ok(())
}

pub fn decrease_balance(
    storage: &mut dyn Storage,
    owner: &Addr,
    denom: &str,
    amount: Uint128,
) -> StdResult<()> {
    let key = (owner, denom);
    let balance = BALANCES.may_load(storage, key)?.unwrap_or_default();
    if balance < amount {
        return Err(StdError::generic_err("Insufficient funds"));
    }
    BALANCES.save(storage, key, &(balance - amount))?;
    Ok(())
}

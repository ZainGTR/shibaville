#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{BalanceResponse, ExecuteMsg, InstantiateMsg, QueryMsg, TokenInfoResponse};
use crate::state::{increase_balance, decrease_balance, TokenInfo, BALANCES, TOKENS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:resources";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateToken { denom, name, symbol, decimals } => {
            try_create_token(deps, info, denom, name, symbol, decimals)
        }
        ExecuteMsg::Mint { denom, amount } => try_mint(deps, env, info, denom, amount),
        ExecuteMsg::Burn { denom, amount } => try_burn(deps, env, info, denom, amount),
        ExecuteMsg::Transfer { recipient, denom, amount } => {
            try_transfer(deps, info, recipient, denom, amount)
        }
    }
}

fn try_create_token(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
    name: String,
    symbol: String,
    decimals: u8,
) -> Result<Response, ContractError> {
    if TOKENS.has(deps.storage, &denom) {
        return Err(ContractError::TokenAlreadyExists {});
    }

    let token_info = TokenInfo {
        name,
        symbol,
        decimals,
        total_supply: Uint128::zero(),
    };

    TOKENS.save(deps.storage, &denom, &token_info)?;

    Ok(Response::new()
        .add_attribute("method", "create_token")
        .add_attribute("denom", denom))
}

fn try_mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    if !TOKENS.has(deps.storage, &denom) {
        return Err(ContractError::TokenDoesNotExist {});
    }

    let mut token_info = TOKENS.load(deps.storage, &denom)?;
    token_info.total_supply += amount;
    TOKENS.save(deps.storage, &denom, &token_info)?;

    increase_balance(deps.storage, &info.sender, &denom, amount)?;

    Ok(Response::new()
        .add_attribute("method", "mint")
        .add_attribute("denom", denom)
        .add_attribute("amount", amount.to_string()))
}

fn try_burn(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    if !TOKENS.has(deps.storage, &denom) {
        return Err(ContractError::TokenDoesNotExist {});
    }

    let mut token_info = TOKENS.load(deps.storage, &denom)?;
    if token_info.total_supply < amount {
        return Err(ContractError::InsufficientFunds {});
    }
    token_info.total_supply -= amount;
    TOKENS.save(deps.storage, &denom, &token_info)?;

    decrease_balance(deps.storage, &info.sender, &denom, amount)?;

    Ok(Response::new()
        .add_attribute("method", "burn")
        .add_attribute("denom", denom)
        .add_attribute("amount", amount.to_string()))
}

fn try_transfer(
    deps: DepsMut,
    info: MessageInfo,
    recipient: String,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let recipient_addr = deps.api.addr_validate(&recipient)?;

    decrease_balance(deps.storage, &info.sender, &denom, amount)?;
    increase_balance(deps.storage, &recipient_addr, &denom, amount)?;

    Ok(Response::new()
        .add_attribute("method", "transfer")
        .add_attribute("denom", denom)
        .add_attribute("amount", amount.to_string())
        .add_attribute("from", info.sender.to_string())
        .add_attribute("to", recipient))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TokenInfo { denom } => to_json_binary(&query_token_info(deps, denom)?),
        QueryMsg::Balance { owner, denom } => to_json_binary(&query_balance(deps, owner, denom)?),
    }
}

fn query_token_info(deps: Deps, denom: String) -> StdResult<TokenInfoResponse> {
    let token_info = TOKENS.load(deps.storage, &denom)?;
    Ok(TokenInfoResponse {
        name: token_info.name,
        symbol: token_info.symbol,
        decimals: token_info.decimals,
        total_supply: token_info.total_supply,
    })
}

fn query_balance(deps: Deps, owner: String, denom: String) -> StdResult<BalanceResponse> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let balance = BALANCES
        .may_load(deps.storage, (&owner_addr, &denom))?
        .unwrap_or_default();
    Ok(BalanceResponse { balance })
}

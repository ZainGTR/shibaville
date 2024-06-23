#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, WasmMsg,
};

use serde::{Deserialize, Serialize};


use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{Config, Player, CONFIG, PLAYERS, VILLE_COUNT, BUILDING_COUNT};

const CONTRACT_NAME: &str = "crates.io:shibaville";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let admin = deps.api.addr_validate(&msg.admin)?;
    let ville_nft_contract = deps.api.addr_validate(&msg.ville_nft_contract)?;
    let building_nft_contract = deps.api.addr_validate(&msg.building_nft_contract)?;
    let resource_token_contract = deps.api.addr_validate(&msg.resource_token_contract)?;
    let unit_token_contract = deps.api.addr_validate(&msg.unit_token_contract)?;

    let config = Config {
        admin,
        ville_nft_contract,
        building_nft_contract,
        resource_token_contract,
        unit_token_contract,
    };
    CONFIG.save(deps.storage, &config)?;

    VILLE_COUNT.save(deps.storage, &0u64)?;
    BUILDING_COUNT.save(deps.storage, &0u64)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisterPlayer { archid } => try_register_player(deps, env, info, archid),
        ExecuteMsg::MintVille { ville_metadata } => try_mint_ville(deps, env, info, ville_metadata),
        ExecuteMsg::MintBuilding { ville_id, building_metadata } => try_mint_building(deps, env, info, ville_id, building_metadata),
        ExecuteMsg::PlaceBuilding { ville_id, building_id } => try_place_building(deps, env, info, ville_id, building_id),
    }
}

fn try_register_player(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    archid: String,
) -> Result<Response, ContractError> {
    let address = info.sender.clone();

    if PLAYERS.has(deps.storage, &address) {
        return Err(ContractError::PlayerAlreadyRegistered {});
    }

    let player = Player {
        address: info.sender.clone(),
        ville_id: None,
    };

    PLAYERS.save(deps.storage, &address, &player)?;

    Ok(Response::new()
        .add_attribute("method", "register_player")
        .add_attribute("player", info.sender.to_string())
        .add_attribute("archid", archid))
}

fn try_mint_ville(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    ville_metadata: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let address = info.sender.clone();

    if !PLAYERS.has(deps.storage, &address) {
        return Err(ContractError::PlayerNotRegistered {});
    }

    let mut player = PLAYERS.load(deps.storage, &address)?;
    if player.ville_id.is_some() {
        return Err(ContractError::VilleAlreadyMinted {});
    }

    let ville_id = VILLE_COUNT.load(deps.storage)? + 1;
    VILLE_COUNT.save(deps.storage, &ville_id)?;

    player.ville_id = Some(ville_id);
    PLAYERS.save(deps.storage, &address, &player)?;

    let mint_msg = MintMsg::<Option<state::Metadata>> {
        token_id: ville_id.to_string(),
        owner: info.sender.to_string(),
        token_uri: None,
        extension: Some(serde_json::from_str(&ville_metadata).unwrap()),
    };

    let exec_msg = WasmMsg::Execute {
        contract_addr: config.ville_nft_contract.to_string(),
        msg: to_json_binary(&cw721_base::ExecuteMsg::Mint(mint_msg))?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(exec_msg)
        .add_attribute("method", "mint_ville")
        .add_attribute("owner", info.sender.to_string())
        .add_attribute("ville_id", ville_id.to_string()))
}

fn try_mint_building(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    ville_id: u64,
    building_metadata: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let address = info.sender.clone();

    let player = PLAYERS.load(deps.storage, &address)?;
    if player.ville_id != Some(ville_id) {
        return Err(ContractError::Unauthorized {});
    }

    let building_id = BUILDING_COUNT.load(deps.storage)? + 1;
    BUILDING_COUNT.save(deps.storage, &building_id)?;

    let mint_msg = MintMsg::<Option<state::Metadata>> {
        token_id: building_id.to_string(),
        owner: info.sender.to_string(),
        token_uri: None,
        extension: Some(serde_json::from_str(&building_metadata).unwrap()),
    };

    let exec_msg = WasmMsg::Execute {
        contract_addr: config.building_nft_contract.to_string(),
        msg: to_json_binary(&cw721_base::ExecuteMsg::Mint(mint_msg))?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(exec_msg)
        .add_attribute("method", "mint_building")
        .add_attribute("owner", info.sender.to_string())
        .add_attribute("building_id", building_id.to_string()))
}

fn try_place_building(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    ville_id: u64,
    building_id: u64,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let address = info.sender.clone();

    let player = PLAYERS.load(deps.storage, &address)?;
    if player.ville_id != Some(ville_id) {
        return Err(ContractError::Unauthorized {});
    }

    // Here you would implement logic to burn resources and mint new resources
    // based on the metadata of the building being placed.

    Ok(Response::new()
        .add_attribute("method", "place_building")
        .add_attribute("ville_id", ville_id.to_string())
        .add_attribute("building_id", building_id.to_string()))
}

use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{PLAYERS, VILLES, BUILDINGS, RESOURCES, Building};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::RegisterPlayer { archid } => try_register_player(deps, info, archid),
        ExecuteMsg::MintBuilding { building_id, ville_id, resource_cost, resource_production } => {
            try_mint_building(deps, info, building_id, ville_id, resource_cost, resource_production)
        }
        ExecuteMsg::PlaceBuilding { building_id } => try_place_building(deps, info, building_id),
        ExecuteMsg::MintResources { amount } => try_mint_resources(deps, info, amount),
        ExecuteMsg::BurnResources { amount } => try_burn_resources(deps, info, amount),
    }
}

fn try_register_player(
    deps: DepsMut,
    info: MessageInfo,
    archid: String,
) -> Result<Response, StdError> {
    if PLAYERS.has(deps.storage, &info.sender) {
        return Err(StdError::generic_err("Player already registered"));
    }

    PLAYERS.save(deps.storage, &info.sender, &archid)?;
    VILLES.save(deps.storage, &archid, &info.sender)?;

    Ok(Response::new().add_attribute("method", "register_player").add_attribute("archid", archid))
}

fn try_mint_building(
    deps: DepsMut,
    info: MessageInfo,
    building_id: String,
    ville_id: String,
    resource_cost: Uint128,
    resource_production: Uint128,
) -> Result<Response, StdError> {
    let player_addr = VILLES.load(deps.storage, &ville_id)?;
    if player_addr != info.sender {
        return Err(StdError::generic_err("Not authorized"));
    }

    let building = Building {
        id: building_id.clone(),
        ville_id: ville_id.clone(),
        resource_cost,
        resource_production,
    };

    BUILDINGS.save(deps.storage, &building_id, &building)?;

    Ok(Response::new()
        .add_attribute("method", "mint_building")
        .add_attribute("building_id", building_id)
        .add_attribute("ville_id", ville_id))
}

fn try_place_building(
    deps: DepsMut,
    info: MessageInfo,
    building_id: String,
) -> Result<Response, StdError> {
    let building = BUILDINGS.load(deps.storage, &building_id)?;
    let player_addr = VILLES.load(deps.storage, &building.ville_id)?;
    if player_addr != info.sender {
        return Err(StdError::generic_err("Not authorized"));
    }

    let mut balance = RESOURCES.load(deps.storage, &info.sender)?;
    if balance < building.resource_cost {
        return Err(StdError::generic_err("Insufficient resources"));
    }

    balance -= building.resource_cost;
    RESOURCES.save(deps.storage, &info.sender, &balance)?;

    Ok(Response::new()
        .add_attribute("method", "place_building")
        .add_attribute("building_id", building_id))
}

fn try_mint_resources(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, StdError> {
    let mut balance = RESOURCES.load(deps.storage, &info.sender).unwrap_or(Uint128::zero());
    balance += amount;
    RESOURCES.save(deps.storage, &info.sender, &balance)?;

    Ok(Response::new()
        .add_attribute("method", "mint_resources")
        .add_attribute("amount", amount.to_string()))
}

fn try_burn_resources(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, StdError> {
    let mut balance = RESOURCES.load(deps.storage, &info.sender).unwrap_or(Uint128::zero());
    if balance < amount {
        return Err(StdError::generic_err("Insufficient resources"));
    }

    balance -= amount;
    RESOURCES.save(deps.storage, &info.sender, &balance)?;

    Ok(Response::new()
        .add_attribute("method", "burn_resources")
        .add_attribute("amount", amount.to_string()))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPlayer { address } => to_binary(&query_player(deps, address)?),
        QueryMsg::GetVille { archid } => to_binary(&query_ville(deps, archid)?),
        QueryMsg::GetBuilding { building_id } => to_binary(&query_building(deps, building_id)?),
        QueryMsg::GetResources { address } => to_binary(&query_resources(deps, address)?),
    }
}

fn query_player(deps: Deps, address: String) -> StdResult<String> {
    let addr = deps.api.addr_validate(&address)?;
    let archid = PLAYERS.load(deps.storage, &addr)?;
    Ok(archid)
}

fn query_ville(deps: Deps, archid: String) -> StdResult<Addr> {
    let addr = VILLES.load(deps.storage, &archid)?;
    Ok(addr)
}

fn query_building(deps: Deps, building_id: String) -> StdResult<Building> {
    let building = BUILDINGS.load(deps.storage, &building_id)?;
    Ok(building)
}

fn query_resources(deps: Deps, address: String) -> StdResult<Uint128> {
    let addr = deps.api.addr_validate(&address)?;
    let balance = RESOURCES.load(deps.storage, &addr).unwrap_or(Uint128::zero());
    Ok(balance)
}

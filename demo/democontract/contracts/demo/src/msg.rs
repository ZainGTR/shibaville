use cosmwasm_std::Uint128;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {
    RegisterPlayer { archid: String },
    MintBuilding { building_id: String, resource_cost: Uint128, resource_production: Uint128 },
    PlaceBuilding { building_id: String },
    MintResources { amount: Uint128 },
    BurnResources { amount: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QueryMsg {
    GetPlayer { address: String },
    GetVille { archid: String },
    GetBuilding { building_id: String },
    GetResources { address: String },
}

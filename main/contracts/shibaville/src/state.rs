use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    pub admin: Addr,
    pub ville_nft_contract: Addr,
    pub building_nft_contract: Addr,
    pub resource_token_contract: Addr,
    pub unit_token_contract: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Player {
    pub address: Addr,
    pub ville_id: Option<u64>,
}

pub const PLAYERS: Map<&Addr, Player> = Map::new("players");
pub const VILLE_COUNT: Item<u64> = Item::new("ville_count");
pub const BUILDING_COUNT: Item<u64> = Item::new("building_count");

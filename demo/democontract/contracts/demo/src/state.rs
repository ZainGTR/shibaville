use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

pub const PLAYERS: Map<&Addr, String> = Map::new("players"); // Maps player address to archid
pub const VILLES: Map<&String, Addr> = Map::new("villes"); // Maps archid to player address

#[derive(Clone, Debug, PartialEq)]
pub struct Building {
    pub id: String,
    pub ville_id: String,
    pub resource_cost: Uint128,
    pub resource_production: Uint128,
}

pub const BUILDINGS: Map<&String, Building> = Map::new("buildings"); // Maps building id to Building details
pub const RESOURCES: Map<&Addr, Uint128> = Map::new("resources"); // Maps player address to their resource balance


use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub admin: String,
    pub ville_nft_contract: String,
    pub building_nft_contract: String,
    pub resource_token_contract: String,
    pub unit_token_contract: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {
    RegisterPlayer { archid: String },
    MintVille { ville_metadata: String },
    MintBuilding { ville_id: u64, building_metadata: String },
    PlaceBuilding { ville_id: u64, building_id: u64 },
}

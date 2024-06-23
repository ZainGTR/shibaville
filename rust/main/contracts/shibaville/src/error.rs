use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    
    #[error("PlayerAlreadyRegistered")]
    PlayerAlreadyRegistered {},
    
    #[error("PlayerNotRegistered")]
    PlayerNotRegistered {},
    
    #[error("VilleAlreadyMinted")]
    VilleAlreadyMinted {},
}

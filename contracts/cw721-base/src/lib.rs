pub mod contract;
mod error;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};

    // This is a simple type to let us handle empty extensions
    pub type Extension = Option<Empty>;

    // This makes a conscious choice on the various generics used by the contract
    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: msg::InstantiateMsg,
    ) -> StdResult<Response> {
        let tract = contract::Cw721Contract::<Extension, Empty>::default();
        tract.instantiate(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: msg::ExecuteMsg<Extension>,
    ) -> Result<Response, ContractError> {
        let tract = contract::Cw721Contract::<Extension, Empty>::default();
        tract.execute(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
        let tract = contract::Cw721Contract::<Extension, Empty>::default();
        tract.query(deps, env, msg)
    }
}

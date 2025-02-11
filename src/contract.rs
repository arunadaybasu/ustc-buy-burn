use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::burn::{execute_swap_and_burn, query_burn};
use crate::staking::{execute_stake, query_stake};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // You can use msg.initial_balance here if needed
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Stake { amount } => execute_stake(deps, env, info, ExecuteMsg::Stake { amount }),
        ExecuteMsg::SwapAndBurn {} => execute_swap_and_burn(deps, env, info, ExecuteMsg::SwapAndBurn {}),
        ExecuteMsg::ClaimRewards {} => Err(cosmwasm_std::StdError::generic_err("Not implemented")),
        ExecuteMsg::UpdateRatios { .. } => Err(cosmwasm_std::StdError::generic_err("Not implemented")),
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStakedBalance { address } => query_stake(deps, env, QueryMsg::GetStakedBalance { address }),
        QueryMsg::GetBurnedUstc {} => query_burn(deps, env, msg),
        QueryMsg::GetConfig {} => query_burn(deps, env, msg),
    }
} 
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, Uint128, StdResult, to_json_binary, Binary};
use cw_storage_plus::Item;
use crate::msg::{ExecuteMsg, QueryMsg};

const STAKED_LUNC: Item<Uint128> = Item::new("staked_lunc");

pub fn execute_stake(deps: DepsMut, _env: Env, _info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    if let ExecuteMsg::Stake { amount } = msg {
        let staked = STAKED_LUNC.load(deps.storage)?;
        STAKED_LUNC.save(deps.storage, &(staked + amount))?;
        Ok(Response::new()
            .add_attribute("action", "stake")
            .add_attribute("amount", amount))
    } else {
        Err(cosmwasm_std::StdError::generic_err("Invalid message type"))
    }
}

pub fn query_stake(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStakedBalance { address: _ } => {
            let balance = STAKED_LUNC.load(deps.storage)?;
            to_json_binary(&balance)
        },
        QueryMsg::GetBurnedUstc {} => to_json_binary(&"Burned USTC not tracked yet"),
        QueryMsg::GetConfig {} => to_json_binary(&"Config not implemented"),
    }
}

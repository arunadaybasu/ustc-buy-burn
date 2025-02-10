use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128, StdResult};
use cw_storage_plus::Item;

const STAKED_LUNC: Item<Uint128> = Item::new("staked_lunc");
const REWARD_POOL: Item<Uint128> = Item::new("reward_pool");

pub fn execute_stake(deps: DepsMut, _env: Env, info: MessageInfo) -> StdResult<Response> {
    let amount = info.funds.iter().find(|c| c.denom == "ulunc").map(|c| c.amount);
    if let Some(amount) = amount {
        let staked = STAKED_LUNC.load(deps.storage)?;
        STAKED_LUNC.save(deps.storage, &(staked + amount))?;
        Ok(Response::new().add_attribute("action", "stake").add_attribute("amount", amount))
    } else {
        Err(cosmwasm_std::StdError::generic_err("No LUNC sent"))
    }
}

use cosmwasm_std::{coin, CosmosMsg, DepsMut, Env, Response, Uint128, BankMsg, StdResult};
use cw_storage_plus::Item;

const BURN_ADDRESS: &str = "terra1burnaddress";
const LUNC_FUNDS: Item<Uint128> = Item::new("lunc_funds");

pub fn execute_swap_and_burn(deps: DepsMut, env: Env) -> StdResult<Response> {
    let funds = LUNC_FUNDS.load(deps.storage)?;
    if funds.is_zero() {
        return Err(cosmwasm_std::StdError::generic_err("No funds available to swap"));
    }

    let swap_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: "dex_address".into(),
        amount: vec![coin(funds.u128(), "ulunc")],
    });

    let burn_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: BURN_ADDRESS.into(),
        amount: vec![coin(funds.multiply_ratio(Uint128::from(98u128), Uint128::from(100u128)).u128(), "uusd")],
    });

    LUNC_FUNDS.save(deps.storage, &Uint128::zero())?;

    Ok(Response::new()
        .add_message(swap_msg)
        .add_message(burn_msg)
        .add_attribute("action", "swap_and_burn")
        .add_attribute("swapped", funds)
        .add_attribute("burned", funds.multiply_ratio(Uint128::from(98u128), Uint128::from(100u128))))
}

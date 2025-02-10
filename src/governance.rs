use cosmwasm_std::{DepsMut, Env, Response, StdResult};
use cw_storage_plus::Item;

const BURN_RATIO: Item<u8> = Item::new("burn_ratio");
const RESTAKE_RATIO: Item<u8> = Item::new("restake_ratio");
const DEV_FEE: Item<u8> = Item::new("dev_fee");
const INFRA_FEE: Item<u8> = Item::new("infra_fee");

pub fn update_ratios(deps: DepsMut, _env: Env, burn_ratio: u8, restake_ratio: u8, dev_fee: u8, infra_fee: u8) -> StdResult<Response> {
    BURN_RATIO.save(deps.storage, &burn_ratio)?;
    RESTAKE_RATIO.save(deps.storage, &restake_ratio)?;
    DEV_FEE.save(deps.storage, &dev_fee)?;
    INFRA_FEE.save(deps.storage, &infra_fee)?;

    Ok(Response::new().add_attribute("action", "update_ratios"))
}

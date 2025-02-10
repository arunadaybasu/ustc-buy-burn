use cosmwasm_std::Uint128;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Stake { amount: Uint128 },
    ClaimRewards {},
    SwapAndBurn {},
    UpdateRatios {
        burn_ratio: u8,
        restake_ratio: u8,
        dev_fee: u8,
        infra_fee: u8,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetStakedBalance { address: String },
    GetBurnedUstc {},
    GetConfig {},
}

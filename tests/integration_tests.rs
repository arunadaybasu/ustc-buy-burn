#[cfg(test)]
mod tests {
    use cosmwasm_std::{coin, Addr, Response, StdError, Uint128, to_json_binary};
    use cw_multi_test::{App, ContractWrapper, Executor};
    use lunc_ustc_burn_project::contract::{execute, instantiate, query};
    use lunc_ustc_burn_project::msg::{ExecuteMsg, InstantiateMsg};

    #[test]
    fn test_staking_lunc() {
        let mut app = App::default();
        let sender = Addr::unchecked("terra1user");

        let contract = ContractWrapper::new(instantiate, execute, query)
            .with_reply::<StdError>(|_deps, _env, _reply| Ok(Response::default()));
        let code_id = app.store_code(Box::new(contract));

        let instantiate_msg = InstantiateMsg {};
        let contract_addr = app.instantiate_contract(
            code_id,
            sender.clone(),
            &to_json_binary(&instantiate_msg).unwrap(),
            &[],
            "LUNC Staking",
            None,
        )
        .unwrap();

        let msg = to_json_binary(&ExecuteMsg::Stake { amount: Uint128::new(1000000) }).unwrap();
        let funds = vec![coin(1000000, "ulunc")];

        let res = app.execute_contract(sender.clone(), contract_addr.clone(), &msg, &funds);
        assert!(res.is_ok());
    }

    #[test]
    fn test_swap_and_burn() {
        let mut app = App::default();
        let sender = Addr::unchecked("terra1user");

        let contract = ContractWrapper::new(instantiate, execute, query)
            .with_reply::<StdError>(|_deps, _env, _reply| Ok(Response::default()));
        let code_id = app.store_code(Box::new(contract));

        let instantiate_msg = InstantiateMsg {};
        let contract_addr = app.instantiate_contract(
            code_id,
            sender.clone(),
            &to_json_binary(&instantiate_msg).unwrap(),
            &[],
            "USTC Burner",
            None,
        )
        .unwrap();

        let msg = to_json_binary(&ExecuteMsg::SwapAndBurn {}).unwrap();
        let funds = vec![coin(1000000, "ulunc")];

        let res = app.execute_contract(sender.clone(), contract_addr.clone(), &msg, &funds);
        assert!(res.is_ok());
    }
}

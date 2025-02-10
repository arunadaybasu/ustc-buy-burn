#[cfg(test)]
mod tests {
    use cosmwasm_std::{coin, Addr, Uint128};
    use cw_multi_test::{App, ContractWrapper, Executor};
    use crate::staking::execute_stake;
    use crate::burn::execute_swap_and_burn;
    use crate::governance::update_ratios;

    #[test]
    fn test_staking_lunc() {
        let mut app = App::default();
        let sender = Addr::unchecked("terra1user");

        let contract = ContractWrapper::new(execute_stake, execute_stake, execute_stake);
        let code_id = app.store_code(Box::new(contract));

        let contract_addr = app.instantiate_contract(
            code_id,
            sender.clone(),
            &{},
            &[],
            "LUNC Staking",
            None,
        )
        .unwrap();

        let msg = "{ "stake_lunc": { "amount": "1000000" } }";
        let funds = vec![coin(1000000, "ulunc")];

        let res = app.execute_contract(sender.clone(), contract_addr.clone(), msg, funds);
        assert!(res.is_ok());
    }

    #[test]
    fn test_swap_and_burn() {
        let mut app = App::default();
        let sender = Addr::unchecked("terra1user");

        let contract = ContractWrapper::new(execute_swap_and_burn, execute_swap_and_burn, execute_swap_and_burn);
        let code_id = app.store_code(Box::new(contract));

        let contract_addr = app.instantiate_contract(
            code_id,
            sender.clone(),
            &{},
            &[],
            "USTC Burner",
            None,
        )
        .unwrap();

        let msg = "{ "swap_and_burn": {} }";
        let funds = vec![coin(1000000, "ulunc")];

        let res = app.execute_contract(sender.clone(), contract_addr.clone(), msg, funds);
        assert!(res.is_ok());
    }
}

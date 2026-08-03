use super::*;
use soroban_sdk::{Address, Env, String};

#[test]
fn opens_and_updates_record() {
    let env = Env::default();
    let contract_id = env.register(ProofOfPaymentForGovernmentServicesContract, ());
    let client = ProofOfPaymentForGovernmentServicesContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let asset = Address::generate(&env);
    let owner = Address::generate(&env);

    env.mock_all_auths();
    client.initialize(&admin, &asset, &String::from_str(&env, "Proof of Payment for Governmen"));
    client.open_record(&String::from_str(&env, "REC-001"), &owner, &1000);
    client.pay_public_sector_record(&String::from_str(&env, "REC-001"), &owner, &250, &86);

    let record = client.get_record(&String::from_str(&env, "REC-001")).unwrap();
    assert_eq!(record.released_amount, 250);
    assert_eq!(record.score, 86);
}

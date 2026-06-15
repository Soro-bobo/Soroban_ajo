#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Events},
    Address, Env, String,
};

use ajo_contract::{AjoContract, AjoContractClient};

fn setup_env() -> (Env, AjoContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(AjoContract, ());
    let client = AjoContractClient::new(&env, &contract_id);
    (env, client)
}

fn alice(env: &Env) -> Address {
    Address::generate(env)
}

fn group_name(env: &Env) -> String {
    String::from_str(env, "Lagos Savings Circle")
}

#[test]
fn test_initialize() {
    let (env, client) = setup_env();
    let admin = alice(&env);
    client.initialize(&admin);
}

#[test]
#[should_panic]
fn test_initialize_twice_panics() {
    let (env, client) = setup_env();
    let admin = alice(&env);
    client.initialize(&admin);
    client.initialize(&admin);
}

#[test]
fn test_create_group() {
    let (env, client) = setup_env();
    let admin = alice(&env);
    client.initialize(&admin);

    let creator = alice(&env);
    let group_id = client.create_group(
        &creator,
        &group_name(&env),
        &100_0000000_i128, // 100 XLM in stroops
        &ajo_contract::types::Frequency::Monthly,
        &5_u32,
        &1000_u32,
    );

    assert_eq!(group_id, 1u64);

    let group = client.get_group(&group_id);
    assert_eq!(group.member_count, 1);
    assert_eq!(group.contribution_amount, 100_0000000);
    assert_eq!(group.max_members, 5);

    let member = client.get_member(&group_id, &creator);
    assert_eq!(member.payout_position, 1);
    assert_eq!(member.has_received_payout, false);
}

#[test]
fn test_join_group() {
    let (env, client) = setup_env();
    let admin = alice(&env);
    client.initialize(&admin);

    let creator = alice(&env);
    let group_id = client.create_group(
        &creator,
        &group_name(&env),
        &50_0000000_i128,
        &ajo_contract::types::Frequency::Weekly,
        &3_u32,
        &100_u32,
    );

    let member2 = alice(&env);
    let position = client.join_group(&group_id, &member2);
    assert_eq!(position, 2);

    let member3 = alice(&env);
    let position3 = client.join_group(&group_id, &member3);
    assert_eq!(position3, 3);

    let group = client.get_group(&group_id);
    assert_eq!(group.member_count, 3);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn test_join_full_group_panics() {
    let (env, client) = setup_env();
    let admin = alice(&env);
    client.initialize(&admin);

    let creator = alice(&env);
    let group_id = client.create_group(
        &creator,
        &group_name(&env),
        &10_0000000_i128,
        &ajo_contract::types::Frequency::Monthly,
        &2_u32,
        &100_u32,
    );

    let m2 = alice(&env);
    client.join_group(&group_id, &m2);

    // Third join should fail — group is full
    let m3 = alice(&env);
    client.join_group(&group_id, &m3);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_double_join_panics() {
    let (env, client) = setup_env();
    let admin = alice(&env);
    client.initialize(&admin);

    let creator = alice(&env);
    let group_id = client.create_group(
        &creator,
        &group_name(&env),
        &10_0000000_i128,
        &ajo_contract::types::Frequency::Weekly,
        &5_u32,
        &100_u32,
    );

    let member = alice(&env);
    client.join_group(&group_id, &member);
    client.join_group(&group_id, &member); // should panic
}

#[test]
fn test_contribute() {
    let (env, client) = setup_env();
    let admin = alice(&env);
    client.initialize(&admin);

    let creator = alice(&env);
    let group_id = client.create_group(
        &creator,
        &group_name(&env),
        &100_0000000_i128,
        &ajo_contract::types::Frequency::Monthly,
        &3_u32,
        &100_u32,
    );

    client.activate_group(&group_id, &creator);

    let total = client.contribute(&group_id, &creator, &100_0000000_i128);
    assert_eq!(total, 100_0000000);

    let member = client.get_member(&group_id, &creator);
    assert_eq!(member.total_contributed, 100_0000000);
}

#[test]
#[should_panic(expected = "Error(Contract, #7)")]
fn test_contribute_below_minimum_panics() {
    let (env, client) = setup_env();
    let admin = alice(&env);
    client.initialize(&admin);

    let creator = alice(&env);
    let group_id = client.create_group(
        &creator,
        &group_name(&env),
        &100_0000000_i128,
        &ajo_contract::types::Frequency::Monthly,
        &3_u32,
        &100_u32,
    );

    client.activate_group(&group_id, &creator);
    client.contribute(&group_id, &creator, &50_0000000_i128); // below minimum
}

#[test]
fn test_activate_and_distribute_payout() {
    let (env, client) = setup_env();
    let admin = alice(&env);
    client.initialize(&admin);

    let creator = alice(&env);
    let group_id = client.create_group(
        &creator,
        &group_name(&env),
        &100_0000000_i128,
        &ajo_contract::types::Frequency::Monthly,
        &2_u32,
        &100_u32,
    );

    let m2 = alice(&env);
    client.join_group(&group_id, &m2);

    client.activate_group(&group_id, &creator);

    let group = client.get_group(&group_id);
    assert_eq!(group.status, ajo_contract::types::GroupStatus::Active);

    let recipient = client.distribute_payout(&group_id, &creator);
    assert_eq!(recipient, creator);
}

#[test]
fn test_get_member_position() {
    let (env, client) = setup_env();
    let admin = alice(&env);
    client.initialize(&admin);

    let creator = alice(&env);
    let group_id = client.create_group(
        &creator,
        &group_name(&env),
        &10_0000000_i128,
        &ajo_contract::types::Frequency::Weekly,
        &5_u32,
        &100_u32,
    );

    let m2 = alice(&env);
    client.join_group(&group_id, &m2);

    assert_eq!(client.get_member_position(&group_id, &creator), 1);
    assert_eq!(client.get_member_position(&group_id, &m2), 2);
}

#[test]
fn test_events_emitted_on_create() {
    let (env, client) = setup_env();
    let admin = alice(&env);
    client.initialize(&admin);

    let creator = alice(&env);
    client.create_group(
        &creator,
        &group_name(&env),
        &100_0000000_i128,
        &ajo_contract::types::Frequency::Monthly,
        &5_u32,
        &1000_u32,
    );

    let events = env.events().all();
    assert!(events.len() >= 2, "Expected at least GroupCreated and MemberJoined events");
}

use soroban_sdk::{contracttype, Address, Env};

use crate::types::{Group, Member};

// Instance storage keys (contract-level config, cheap to access)
#[contracttype]
pub enum InstanceKey {
    GroupCounter,
    Admin,
}

// Persistent storage keys (group/member state, survives archiving)
#[contracttype]
pub enum DataKey {
    Group(u64),
    Member(u64, Address),
    GroupMemberCount(u64),
}

const INSTANCE_BUMP_AMOUNT: u32 = 100;
const PERSISTENT_BUMP_AMOUNT: u32 = 500;

pub fn get_group_counter(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&InstanceKey::GroupCounter)
        .unwrap_or(0u64)
}

pub fn increment_group_counter(env: &Env) -> u64 {
    let next = get_group_counter(env) + 1;
    env.storage()
        .instance()
        .set(&InstanceKey::GroupCounter, &next);
    next
}

pub fn save_group(env: &Env, group: &Group) {
    env.storage()
        .persistent()
        .set(&DataKey::Group(group.id), group);
    env.storage()
        .persistent()
        .extend_ttl(&DataKey::Group(group.id), PERSISTENT_BUMP_AMOUNT, PERSISTENT_BUMP_AMOUNT);
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_BUMP_AMOUNT, INSTANCE_BUMP_AMOUNT);
}

pub fn load_group(env: &Env, group_id: u64) -> Option<Group> {
    env.storage()
        .persistent()
        .get(&DataKey::Group(group_id))
}

pub fn save_member(env: &Env, member: &Member) {
    let key = DataKey::Member(member.group_id, member.address.clone());
    env.storage().persistent().set(&key, member);
    env.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_BUMP_AMOUNT, PERSISTENT_BUMP_AMOUNT);
}

pub fn load_member(env: &Env, group_id: u64, address: &Address) -> Option<Member> {
    env.storage()
        .persistent()
        .get(&DataKey::Member(group_id, address.clone()))
}

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&InstanceKey::Admin)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&InstanceKey::Admin, admin);
}

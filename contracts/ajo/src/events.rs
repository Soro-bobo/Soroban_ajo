use soroban_sdk::{symbol_short, Address, Env};

pub fn group_created(env: &Env, group_id: u64, creator: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("group"), symbol_short!("created")),
        (group_id, creator.clone(), amount),
    );
}

pub fn member_joined(env: &Env, group_id: u64, member: &Address, position: u32) {
    env.events().publish(
        (symbol_short!("member"), symbol_short!("joined")),
        (group_id, member.clone(), position),
    );
}

pub fn contribution_made(env: &Env, group_id: u64, contributor: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("contrib"), symbol_short!("made")),
        (group_id, contributor.clone(), amount),
    );
}

pub fn payout_distributed(env: &Env, group_id: u64, recipient: &Address, amount: i128, round: u32) {
    env.events().publish(
        (symbol_short!("payout"), symbol_short!("sent")),
        (group_id, recipient.clone(), amount, round),
    );
}

pub fn group_status_changed(env: &Env, group_id: u64, new_status: &str) {
    env.events().publish(
        (symbol_short!("group"), symbol_short!("status")),
        (group_id, soroban_sdk::String::from_str(env, new_status)),
    );
}

use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum GroupStatus {
    Pending,
    Active,
    Completed,
    Paused,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Frequency {
    Weekly,
    Biweekly,
    Monthly,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Group {
    pub id: u64,
    pub name: String,
    pub contribution_amount: i128,
    pub frequency: Frequency,
    pub max_members: u32,
    pub status: GroupStatus,
    pub start_ledger: u32,
    pub creator: Address,
    pub current_payout_position: u32,
    pub member_count: u32,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Member {
    pub group_id: u64,
    pub address: Address,
    pub payout_position: u32,
    pub has_received_payout: bool,
    pub total_contributed: i128,
    pub joined_ledger: u32,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Contribution {
    pub group_id: u64,
    pub contributor: Address,
    pub amount: i128,
    pub ledger: u32,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct PayoutRecord {
    pub group_id: u64,
    pub recipient: Address,
    pub amount: i128,
    pub round: u32,
    pub ledger: u32,
}

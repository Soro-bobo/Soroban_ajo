#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, String};

mod errors;
mod events;
mod storage;
pub mod types;

use errors::ContractError;
use storage::{
    increment_group_counter, load_group, load_member, load_position_index, save_group, save_member,
    save_position_index, set_admin,
};
use types::{Frequency, Group, GroupStatus, Member};

#[contract]
pub struct AjoContract;

#[contractimpl]
impl AjoContract {
    /// One-time initialization. Sets the contract admin.
    pub fn initialize(env: Env, admin: Address) -> Result<(), ContractError> {
        if storage::get_admin(&env).is_some() {
            return Err(ContractError::AlreadyInitialized);
        }
        admin.require_auth();
        set_admin(&env, &admin);
        Ok(())
    }

    /// Create a new savings group. Returns the new group ID.
    pub fn create_group(
        env: Env,
        creator: Address,
        name: String,
        contribution_amount: i128,
        frequency: Frequency,
        max_members: u32,
        start_ledger: u32,
    ) -> Result<u64, ContractError> {
        creator.require_auth();

        if contribution_amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }
        if !(2..=50).contains(&max_members) {
            return Err(ContractError::InvalidMaxMembers);
        }

        let group_id = increment_group_counter(&env);

        let group = Group {
            id: group_id,
            name,
            contribution_amount,
            frequency,
            max_members,
            status: GroupStatus::Pending,
            start_ledger,
            creator: creator.clone(),
            current_payout_position: 0,
            member_count: 1,
        };

        save_group(&env, &group);

        // Creator is position 1
        let creator_member = Member {
            group_id,
            address: creator.clone(),
            payout_position: 1,
            has_received_payout: false,
            total_contributed: 0,
            joined_ledger: env.ledger().sequence(),
        };
        save_member(&env, &creator_member);
        save_position_index(&env, group_id, 1, &creator);

        events::group_created(&env, group_id, &creator, contribution_amount);
        events::member_joined(&env, group_id, &creator, 1);

        Ok(group_id)
    }

    /// Join an existing group. Returns the member's payout position.
    pub fn join_group(env: Env, group_id: u64, member: Address) -> Result<u32, ContractError> {
        member.require_auth();

        let mut group = load_group(&env, group_id).ok_or(ContractError::GroupNotFound)?;

        if group.status != GroupStatus::Pending && group.status != GroupStatus::Active {
            return Err(ContractError::GroupNotPending);
        }

        if group.member_count >= group.max_members {
            return Err(ContractError::GroupFull);
        }

        if load_member(&env, group_id, &member).is_some() {
            return Err(ContractError::AlreadyMember);
        }

        group.member_count += 1;
        let position = group.member_count;
        save_group(&env, &group);

        let new_member = Member {
            group_id,
            address: member.clone(),
            payout_position: position,
            has_received_payout: false,
            total_contributed: 0,
            joined_ledger: env.ledger().sequence(),
        };
        save_member(&env, &new_member);
        save_position_index(&env, group_id, position, &member);

        events::member_joined(&env, group_id, &member, position);

        Ok(position)
    }

    /// Record a contribution from a member. Token transfer is handled off-chain;
    /// this records the on-chain acknowledgment.
    pub fn contribute(
        env: Env,
        group_id: u64,
        contributor: Address,
        amount: i128,
    ) -> Result<i128, ContractError> {
        contributor.require_auth();

        let group = load_group(&env, group_id).ok_or(ContractError::GroupNotFound)?;

        if group.status != GroupStatus::Active {
            return Err(ContractError::GroupNotActive);
        }

        if amount < group.contribution_amount {
            return Err(ContractError::InsufficientContribution);
        }

        let mut member =
            load_member(&env, group_id, &contributor).ok_or(ContractError::NotMember)?;

        member.total_contributed += amount;
        save_member(&env, &member);

        events::contribution_made(&env, group_id, &contributor, amount);

        Ok(member.total_contributed)
    }

    /// Distribute the payout to the current round's recipient.
    /// The actual token transfer must be done by the backend/caller.
    /// Only the group creator or contract admin may trigger a distribution.
    pub fn distribute_payout(
        env: Env,
        group_id: u64,
        caller: Address,
    ) -> Result<Address, ContractError> {
        caller.require_auth();

        let mut group = load_group(&env, group_id).ok_or(ContractError::GroupNotFound)?;

        if caller != group.creator && storage::get_admin(&env) != Some(caller.clone()) {
            return Err(ContractError::Unauthorized);
        }

        if group.status != GroupStatus::Active {
            return Err(ContractError::GroupNotActive);
        }

        if group.current_payout_position >= group.member_count {
            return Err(ContractError::RoundNotComplete);
        }

        let next_position = group.current_payout_position + 1;

        let recipient_address = load_position_index(&env, group_id, next_position)
            .ok_or(ContractError::MemberNotFound)?;
        let mut recipient =
            load_member(&env, group_id, &recipient_address).ok_or(ContractError::MemberNotFound)?;

        if recipient.has_received_payout {
            return Err(ContractError::PayoutAlreadyReceived);
        }

        recipient.has_received_payout = true;
        save_member(&env, &recipient);

        group.current_payout_position = next_position;

        if group.current_payout_position >= group.member_count {
            group.status = GroupStatus::Completed;
            events::group_status_changed(&env, group_id, "completed");
        }

        save_group(&env, &group);

        let payout_amount = group.contribution_amount * group.member_count as i128;
        events::payout_distributed(
            &env,
            group_id,
            &recipient_address,
            payout_amount,
            next_position,
        );

        Ok(recipient_address)
    }

    /// Activate a pending group (called by creator once group is full or ready).
    pub fn activate_group(env: Env, group_id: u64, creator: Address) -> Result<(), ContractError> {
        creator.require_auth();

        let mut group = load_group(&env, group_id).ok_or(ContractError::GroupNotFound)?;

        if group.creator != creator {
            return Err(ContractError::NotGroupCreator);
        }

        if group.status != GroupStatus::Pending {
            return Err(ContractError::GroupNotPending);
        }

        group.status = GroupStatus::Active;
        save_group(&env, &group);

        events::group_status_changed(&env, group_id, "active");
        Ok(())
    }

    /// Read-only: returns the group state.
    pub fn get_group(env: Env, group_id: u64) -> Result<Group, ContractError> {
        load_group(&env, group_id).ok_or(ContractError::GroupNotFound)
    }

    /// Read-only: returns the member state.
    pub fn get_member(env: Env, group_id: u64, member: Address) -> Result<Member, ContractError> {
        load_member(&env, group_id, &member).ok_or(ContractError::MemberNotFound)
    }

    /// Read-only: returns the payout position of a member.
    pub fn get_member_position(
        env: Env,
        group_id: u64,
        member: Address,
    ) -> Result<u32, ContractError> {
        let m = load_member(&env, group_id, &member).ok_or(ContractError::MemberNotFound)?;
        Ok(m.payout_position)
    }

    /// Read-only: returns total number of groups ever created.
    pub fn get_group_count(env: Env) -> u64 {
        storage::get_group_counter(&env)
    }

    /// Read-only: returns true if a member has already received their payout.
    pub fn has_received_payout(
        env: Env,
        group_id: u64,
        member: Address,
    ) -> Result<bool, ContractError> {
        let m = load_member(&env, group_id, &member).ok_or(ContractError::MemberNotFound)?;
        Ok(m.has_received_payout)
    }

    /// Read-only: returns total amount a member has contributed to a group.
    pub fn get_total_contributed(
        env: Env,
        group_id: u64,
        member: Address,
    ) -> Result<i128, ContractError> {
        let m = load_member(&env, group_id, &member).ok_or(ContractError::MemberNotFound)?;
        Ok(m.total_contributed)
    }

    /// Read-only: returns the address of the member holding a given payout position.
    pub fn get_payout_recipient(
        env: Env,
        group_id: u64,
        position: u32,
    ) -> Result<Address, ContractError> {
        load_position_index(&env, group_id, position).ok_or(ContractError::MemberNotFound)
    }
}

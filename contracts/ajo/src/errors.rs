use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ContractError {
    GroupNotFound = 1,
    MemberNotFound = 2,
    AlreadyMember = 3,
    GroupFull = 4,
    GroupNotActive = 5,
    GroupNotPending = 6,
    InsufficientContribution = 7,
    PayoutAlreadyReceived = 8,
    NotGroupCreator = 9,
    NotMember = 10,
    InvalidAmount = 11,
    InvalidMaxMembers = 12,
    PayoutNotDue = 13,
    AlreadyInitialized = 14,
    RoundNotComplete = 15,
}

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Proposal {
    pub id: u64,
    pub author: Pubkey,
    #[max_len(200)]
    pub content: String,
    pub max_votes: u64,
    pub votes_for: u64,
    pub votes_against: u64,
    pub end_time: i64,
}

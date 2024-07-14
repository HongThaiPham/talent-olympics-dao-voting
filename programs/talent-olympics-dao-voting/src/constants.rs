use anchor_lang::prelude::*;

pub const DISCRIMINATOR_SIZE: usize = std::mem::size_of::<u64>();

#[constant]
pub const PROPOSAL_SEED: &[u8] = b"proposal";

#[constant]
pub const MEMBER_SEED: &[u8] = b"member";

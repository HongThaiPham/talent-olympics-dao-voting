use anchor_lang::prelude::*;

use crate::{Proposal, DISCRIMINATOR_SIZE, PROPOSAL_SEED};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct InitProposal<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [PROPOSAL_SEED, id.to_le_bytes().as_ref()],
        bump,
        space = DISCRIMINATOR_SIZE + Proposal::INIT_SPACE
    )]
    pub proposal: Account<'info, Proposal>,
    pub system_program: Program<'info, System>,
}
impl<'info> InitProposal<'info> {
    pub fn handler(
        &mut self,
        id: u64,
        content: String,
        max_votes: u64,
        end_time: i64,
    ) -> Result<()> {
        self.proposal.set_inner(Proposal {
            id,
            max_votes,
            author: self.signer.to_account_info().key(),
            content,
            votes_for: 0,
            votes_against: 0,
            end_time,
        });
        Ok(())
    }
}

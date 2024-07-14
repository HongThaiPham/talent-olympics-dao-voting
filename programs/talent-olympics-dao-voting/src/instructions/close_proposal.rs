use anchor_lang::prelude::*;

use crate::{error::MyErrorCode, Proposal, PROPOSAL_SEED};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CloseProposal<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [PROPOSAL_SEED, id.to_le_bytes().as_ref()],
        bump,
        constraint = proposal.author == signer.key(),
        close = signer
    )]
    pub proposal: Account<'info, Proposal>,
}

impl<'info> CloseProposal<'info> {
    pub fn handler(&mut self, _id: u64) -> Result<()> {
        require!(
            self.proposal.end_time.lt(&Clock::get()?.unix_timestamp),
            MyErrorCode::ProposalNotExpired
        );
        Ok(())
    }
}

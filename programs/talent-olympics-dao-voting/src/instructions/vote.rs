use anchor_lang::prelude::*;

use crate::{error::MyErrorCode, Member, Proposal, DISCRIMINATOR_SIZE, MEMBER_SEED, PROPOSAL_SEED};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Vote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(
        mut,
        seeds = [PROPOSAL_SEED, id.to_le_bytes().as_ref()],
        bump,
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(
        init_if_needed,
        payer = voter,
        seeds = [MEMBER_SEED, voter.key().as_ref()],
        space = DISCRIMINATOR_SIZE + Member::INIT_SPACE,
        bump
    )]
    pub member: Account<'info, Member>,
    pub system_program: Program<'info, System>,
}

impl<'info> Vote<'info> {
    pub fn handler(&mut self, _id: u64, is_agains: bool) -> Result<()> {
        require!(
            self.proposal.votes_for + self.proposal.votes_against < self.proposal.max_votes,
            MyErrorCode::MaxVotesReached
        );

        if is_agains {
            self.proposal.votes_against = self
                .proposal
                .votes_against
                .checked_add(1)
                .ok_or(MyErrorCode::Overflow)?;
        } else {
            self.proposal.votes_for = self
                .proposal
                .votes_for
                .checked_add(1)
                .ok_or(MyErrorCode::Overflow)?;
        }

        self.member.add_points(1)?;
        Ok(())
    }
}

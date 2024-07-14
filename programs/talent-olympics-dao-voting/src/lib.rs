pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;
pub use state::*;

declare_id!("FwfMnAZT4SJPNN1Zr1x9wBL8sFUJXbdfnjjKYGs2LxEF");

#[program]
pub mod talent_olympics_dao_voting {
    use super::*;

    pub fn init_proposal(
        ctx: Context<InitProposal>,
        id: u64,
        content: String,
        max_votes: u64,
    ) -> Result<()> {
        ctx.accounts.handler(id, content, max_votes)
    }

    pub fn vote(ctx: Context<Vote>, id: u64, is_agains: bool) -> Result<()> {
        ctx.accounts.handler(id, is_agains)
    }
}

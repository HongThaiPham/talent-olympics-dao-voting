pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FwfMnAZT4SJPNN1Zr1x9wBL8sFUJXbdfnjjKYGs2LxEF");

#[program]
pub mod talent_olympics_dao_voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}

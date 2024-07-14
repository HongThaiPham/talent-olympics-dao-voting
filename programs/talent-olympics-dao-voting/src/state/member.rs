use anchor_lang::prelude::*;

use crate::error::MyErrorCode;

#[account]
#[derive(InitSpace)]
pub struct Member {
    pub points: u64,
}

impl Member {
    pub fn add_points(&mut self, points: u64) -> Result<()> {
        self.points = self
            .points
            .checked_add(points)
            .ok_or(MyErrorCode::Overflow)?;
        Ok(())
    }
}

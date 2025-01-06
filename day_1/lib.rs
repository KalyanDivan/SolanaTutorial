use anchor_lang::prelude::*;

declare_id!("CGEpJWwr5XCtrAJedJPnEcdM4uF17wpFKfMMSuyWjr2s");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

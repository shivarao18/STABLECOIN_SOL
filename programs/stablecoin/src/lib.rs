use anchor_lang::prelude::*;

declare_id!("2Qu2o4SphnZBf31NsbkhKgfyJ9Wa5RZJgkEx43tYtDjS");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

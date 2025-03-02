use anchor_lang::prelude::*;

declare_id!("BEv6mLWh28qF15i3CPChEoFgE746Z4Hz4yJdBAC3f759");

#[program]
pub mod dice_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

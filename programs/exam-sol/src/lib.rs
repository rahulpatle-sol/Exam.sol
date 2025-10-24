use anchor_lang::prelude::*;

declare_id!("6Nu6yV4ukuXwcJk4NUvB9KYqjWeeTRwdT7nf2Vfqn6Ag");

#[program]
pub mod exam_sol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

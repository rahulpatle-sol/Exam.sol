use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};
declare_id!("6Nu6yV4ukuXwcJk4NUvB9KYqjWeeTRwdT7nf2Vfqn6Ag");

#[program]
pub mod exam_sol {
    use super::*;

    pub fn initialize_exam(ctx: Context<InitializeExam>,subject:String,pass_mark:u8) -> Result<()> {
             let exam = &mut ctx.accounts.exam;
        exam.pass_mark=pass_mark;
        exam.subject=subject;
        Ok(())
    }
}


pub fn submit_result(ctx:Context<SubmitResult>,score:u8)->Result<()>{


     let result=&mut ctx.accounts.result;
     let exam=&ctx.accounts.exam;


     result.student=ctx.accounts.student.key();
     result.score=score;
     result.passed=score>=exam.pass_mark;


     if result.passed {
            token::mint_to(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo {
                        mint: ctx.accounts.mint.to_account_info(),
                        to: ctx.accounts.token_account.to_account_info(),
                        authority: ctx.accounts.mint_authority.to_account_info(),
                    },
                ),
                1,
            )?;
        }

         Ok(())
}

#[derive(Accounts)]
pub struct InitializeExam<'info> {
    #[account(init, payer = user, space = 8 + 32 + 1)]
    pub exam: Account<'info, Exam>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Exam {
    pub subject: String,
    pub pass_mark: u8,
}

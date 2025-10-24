use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

declare_id!("6Nu6yV4ukuXwcJk4NUvB9KYqjWeeTRwdT7nf2Vfqn6Ag");

#[program]
pub mod exam_sol {
    use super::*;

    pub fn initialize_exam(ctx: Context<InitializeExam>, subject: String, pass_mark: u8) -> Result<()> {
        let exam = &mut ctx.accounts.exam;
        exam.subject = subject;
        exam.pass_mark = pass_mark;
        Ok(())
    }

    pub fn submit_result(ctx: Context<SubmitResult>, score: u8) -> Result<()> {
        let result = &mut ctx.accounts.result;
        let exam = &ctx.accounts.exam;

        result.student = ctx.accounts.student.key();
        result.score = score;
        result.passed = score >= exam.pass_mark;

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
}

#[derive(Accounts)]
pub struct InitializeExam<'info> {
    #[account(init, payer = user, space = 8 + 4 + 32 + 1)]
    pub exam: Account<'info, Exam>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitResult<'info> {
    #[account(mut)]
    pub student: Signer<'info>,
    #[account(init, payer = student, space = 8 + 32 + 1 + 1)]
    pub result: Account<'info, ResultRecord>,
    pub exam: Account<'info, Exam>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    /// CHECK: This is the mint authority and must be a signer for token::mint_to CPI call.
    #[account(mut, signer)]
    pub mint_authority: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Exam {
    pub subject: String,
    pub pass_mark: u8,
}

#[account]
pub struct ResultRecord {
    pub student: Pubkey,
    pub score: u8,
    pub passed: bool,
}

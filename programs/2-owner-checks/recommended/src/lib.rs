use mainstay_lang::prelude::*;
use mainstay_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod owner_checks_recommended {
    use super::*;

    pub fn log_message(ctx: Context<LogMessage>) -> ProgramResult {
        msg!("Your account balance is: {}", ctx.accounts.token.amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    #[account(constraint = authority.key == &token.owner)]
    token: Account<'info, TokenAccount>,
    authority: Signer<'info>,
}

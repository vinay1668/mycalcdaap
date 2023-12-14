use anchor_lang::prelude::*;
use anchor_lang::prelude::Result;

declare_id!("Dzzsa8C76RCT4TDra3UvB8HnsPrLh7tBhUwAHnY9XEou");

#[program]
pub mod mycalcdaap {
    use super::*;
    pub fn create(ctx: Context<Create>, init_message: String) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    } 
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 264)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


// Define the Calculator type
#[account]
pub struct Calculator {
    // Fields of the Calculator type
    pub greeting:String,
    pub result: i64,
    pub remainder:i64,
}

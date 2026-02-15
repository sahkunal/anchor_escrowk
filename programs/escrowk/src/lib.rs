use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("5pybc7Sv9rmBEasFGSv1HiyyTYaefSMosUq6HPSeKJVB");

#[program]
pub mod escrowk {
    use super::*;

    pub fn make(ctx: Context<Make>,seed:u64,deposit:u64, recieve: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, recieve, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)

    }
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
    ctx.accounts.refund_and_close_vault()?;
    Ok(())
}
    pub fn take(ctx: Context<Take>) -> Result<()> {
    ctx.accounts.deposit_to_maker()?;
    ctx.accounts.withdraw_from_vault()?;
    ctx.accounts.close_vault()?;
    Ok(())
}
}

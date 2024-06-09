use anchor_lang::prelude::*;

declare_id!("BC1NCKj237SeYphj1muyR5y4ZxxypEdARyYbtCpYwfLd");

#[program]
pub mod testproject {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

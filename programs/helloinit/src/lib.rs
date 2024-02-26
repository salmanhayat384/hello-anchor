use anchor_lang::prelude::*;

declare_id!("DmbWbQj8c21AzuBkL4DsiyK12bnyVvPGqs2EsLzbjhYi");

#[program]
pub mod helloinit {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

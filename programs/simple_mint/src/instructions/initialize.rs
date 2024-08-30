use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::{constants::*, states::*};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init_if_needed,
        payer = admin,
        seeds = [TRESURE_SEED],
        bump,
        space = 8 + size_of::<Treasure>() + 300
    )]
    pub treasure: Box<Account<'info, Treasure>>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn validate(
        ctx: Context<Self>,
        supply:u64,
        name:String,
        uri:String,
        symbol:String,
        sol_fee:u64,
    ) -> Result<()> {
        let accts: &mut Initialize<'_> = ctx.accounts;
        let admin_key: Pubkey = accts.admin.key();
        accts.treasure.admin = admin_key;
        accts.treasure.supply = supply;
        accts.treasure.name = name;
        accts.treasure.uri = uri;
        accts.treasure.symbol = symbol;
        accts.treasure.sol_fee = sol_fee;
        Ok(())
    }
}

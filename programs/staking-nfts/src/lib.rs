use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::TokenAccount;
use spl_stake_pool::instruction::deposit_sol;
use anchor_spl::token::Token;
use anchor_lang::solana_program::program::invoke_signed;
declare_id!("42YEaUDJnm4byEYgyLrPegAAHAxtJDAbSMMrDb2pTpWP");
const PURCHASER_SEED: &[u8] = b"PURCHASER_SEED";
const TREAUSRY_SEED: &[u8] = b"TREAUSRY_SEED";
#[program]
pub mod staking_nfts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury.load_init()?;
        treasury.authority = *ctx.accounts.authority.key;
        Ok(())
    }
    pub fn purchase_hook(ctx: Context<PurchaseHook>) -> Result<()> {
        let purchaser = &mut ctx.accounts.purchaser.load_init()?;
        
        let mint = &ctx.accounts.mint;
        
        purchaser.time_lock_til = Clock::get()?.unix_timestamp + 60 * 60 * 24 * 7;
        purchaser.mint = mint.key();

        let lamport_snapshot = &ctx.accounts.treasury.to_account_info().lamports();
        let ix = deposit_sol(
            &ctx.accounts.stake_pool_program.key(),
            &ctx.accounts.stake_pool.key(),
            &ctx.accounts.stake_pool_withdraw_authority.key(),
            &ctx.accounts.reserve_stake_account.key(),
            &ctx.accounts.treasury.key(),
            &ctx.accounts.purchaser_token_account.key(),
            &ctx.accounts.manager_fee_account.key(),
            &ctx.accounts.referrer_pool_tokens_account.key(),
            &ctx.accounts.pool_mint.key(),
            &ctx.accounts.token_program.key(),
            *lamport_snapshot - 1000000
        );
        let key = mint.key();
        let seeds = &[PURCHASER_SEED, key.as_ref()];

        invoke_signed(
            &ix,
            &[
                ctx.accounts.stake_pool.to_account_info(),
                ctx.accounts.stake_pool_withdraw_authority.to_account_info(),
                ctx.accounts.reserve_stake_account.to_account_info(),
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.purchaser_token_account.to_account_info(),
                ctx.accounts.manager_fee_account.to_account_info(),
                ctx.accounts.referrer_pool_tokens_account.to_account_info(),
                ctx.accounts.pool_mint.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
            ],
            &[&seeds[..]],
        )?;

        Ok(())
    }
}
#[account(zero_copy)]
pub struct Treasury {
    pub authority: Pubkey,
}
#[account(zero_copy)]
pub struct Purchaser {
    pub mint: Pubkey,
    pub time_lock_til: i64,
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, seeds = [TREAUSRY_SEED], bump , payer = authority, space = 8 + std::mem::size_of::<Treasury>(),)]
    pub treasury: AccountLoader<'info, Treasury>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct PurchaseHook<'info> {
    #[account(init, 
        seeds = [PURCHASER_SEED, mint.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<Purchaser>(),
    )]
    pub purchaser: AccountLoader<'info, Purchaser>,
    #[account(init,
        payer = authority,
        space = 8 + std::mem::size_of::<TokenAccount>(),
    )]
    pub purchaser_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury: AccountLoader<'info, Treasury>,
    #[account(mut)]

    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub mint: Account<'info, Mint>,
    pub stake_pool_program: UncheckedAccount<'info>,
    #[account(mut)]
    pub stake_pool: UncheckedAccount<'info>,
    pub stake_pool_withdraw_authority: AccountInfo<'info>,
    #[account(mut)]
    pub manager_fee_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub referrer_pool_tokens_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub reserve_stake_account: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub pool_mint : Account<'info, Mint>,
}


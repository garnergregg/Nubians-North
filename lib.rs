use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("BG7wQ9qxTr9EHdLkn9weCh5Pmd8BZ22fCy3SEnH97zR2");

#[program]
pub mod cheese_coin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, herd_size: u64) -> Result<()> {
        let herd_info = &mut ctx.accounts.herd_info;
        herd_info.herd_size = herd_size;
        herd_info.last_update = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn mint_tokens(ctx: Context<MintTokens>) -> Result<()> {
        let herd_info = &mut ctx.accounts.herd_info; 
        let mint_amount = herd_info.herd_size * 3200; 
        
        token::mint_to(ctx.accounts.into_mint_to_context(), mint_amount)?;
        
        Ok(())
    }

    pub fn burn_tokens(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
        token::burn(ctx.accounts.into_burn_context(), amount)?;
        Ok(())
    }

    pub fn update_herd_size(ctx: Context<UpdateHerdSize>, new_size: u64) -> Result<()> {
        let herd_info = &mut ctx.accounts.herd_info;
        herd_info.herd_size = new_size;
        herd_info.last_update = Clock::get()?.unix_timestamp; 
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub herd_info: Account<'info, HerdInfo>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub herd_info: Account<'info, HerdInfo>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateHerdSize<'info> {
    #[account(mut)]
    pub herd_info: Account<'info, HerdInfo>,
}

#[account]
pub struct HerdInfo {
    pub herd_size: u64,
    pub last_update: i64,
}

impl<'info> MintTokens<'info> {
    fn into_mint_to_context(self) -> CpiContext<'_, '_, '_, 'info, mint_to<'info>> {
        let cpi_accounts = mint_to {
            mint: self.mint.to_account_info(),
            to: self.to.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(token::id(), cpi_accounts)
    }
}

impl<'info> BurnTokens<'info> {
    fn into_burn_context(self) -> CpiContext<'_, '_, '_, 'info, burn<'info>> {
        let cpi_accounts = burn {
            mint: self.mint.to_account_info(),
            from: self.to.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(token::id(), cpi_accounts)
    }
}

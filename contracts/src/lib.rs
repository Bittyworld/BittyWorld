use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod bittyworld {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, bump: u8) -> Result<()> {
        let game_state = &mut ctx.accounts.game_state;
        game_state.bump = bump;
        game_state.authority = ctx.accounts.authority.key();
        game_state.reward_pool = 0;
        Ok(())
    }

    pub fn distribute_rewards(
        ctx: Context<DistributeRewards>,
        amount: u64,
        winner: Pubkey,
    ) -> Result<()> {
        let game_state = &ctx.accounts.game_state;
        let token_program = &ctx.accounts.token_program;
        let from = &ctx.accounts.from;
        let to = &ctx.accounts.to;

        // Verify authority
        require!(
            game_state.authority == ctx.accounts.authority.key(),
            ErrorCode::Unauthorized
        );

        // Transfer tokens to winner
        token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                token::Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                    authority: game_state.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + GameState::LEN,
        seeds = [b"game_state"],
        bump
    )]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(
        mut,
        seeds = [b"game_state"],
        bump = game_state.bump
    )]
    pub game_state: Account<'info, GameState>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct GameState {
    pub bump: u8,
    pub authority: Pubkey,
    pub reward_pool: u64,
}

impl GameState {
    pub const LEN: usize = 1 + 32 + 8;
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
} 
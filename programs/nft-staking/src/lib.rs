use anchor_lang::prelude::*;

declare_id!("2airia2X7S46jbCgePe1bhZyuMAqBwwyUzRAxWZ5kvEJ");

#[program]
pub mod nft_staking {
    use super::*;

    // Function to stake an NFT
    pub fn stake_nft(ctx: Context<StakeNft>) -> Result<()> {
        let staking_account = &mut ctx.accounts.staking_account;
        staking_account.owner = *ctx.accounts.owner.key;
        staking_account.nft_mint = *ctx.accounts.nft_mint.key;
        Ok(())
    }

    // Function to unstake an NFT
    pub fn unstake_nft(ctx: Context<UnstakeNft>) -> Result<()> {
        let staking_account = &mut ctx.accounts.staking_account;
        require!(staking_account.owner == *ctx.accounts.owner.key, StakingError::Unauthorized);
        staking_account.owner = Pubkey::default();
        staking_account.nft_mint = Pubkey::default();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StakeNft<'info> {
    // Initialize the staking account with a PDA (Program Derived Address)
    #[account(init, payer = owner, space = 8 + 32 + 32, seeds = [b"staking", owner.key().as_ref()], bump)]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    
    /// CHECK: This account is the mint address of the NFT being staked. We are not performing additional checks here because we only need to store the mint address.
    #[account(mut)]
    pub nft_mint: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnstakeNft<'info> {
    // Use the same PDA (Program Derived Address) for the staking account
    #[account(mut, seeds = [b"staking", owner.key().as_ref()], bump)]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct StakingAccount {
    pub owner: Pubkey,
    pub nft_mint: Pubkey,
}

#[error_code]
pub enum StakingError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}

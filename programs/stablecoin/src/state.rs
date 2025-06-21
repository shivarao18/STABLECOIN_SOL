use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Collateral {
    pub depositor: Pubkey,     // depositor wallet address
    pub sol_account: Pubkey,   // depositor pda collateral account (deposit SOL to this account)
    pub token_account: Pubkey, // depositor ata token account (mint stablecoins to this account)
    pub lamport_balance: u64, // current lamport balance of depositor sol_account (for health check calculation)
    pub amount_minted: u64, // current amount stablecoins minted, base unit adjusted for decimal precision (for health check calculation)
    pub bump: u8,           // store bump seed for this collateral account PDA
    pub bump_sol_account: u8, // store bump seed for the  sol_account PDA
    pub is_initialized: bool, // indicate if account data has already been initialized (for check to prevent overriding certain fields)
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Config {
    pub authority: Pubkey,          // authority of the this program config account
    pub mint_account: Pubkey,       // the stablecoin mint address, which is a PDA
    pub liquidation_threshold: u64, // determines how much extra collateral is required
    pub liquidation_bonus: u64,     // % bonus lamports to liquidator for liquidating an account
    pub min_health_factor: u64, // minimum health factor, if below min then Collateral account can be liquidated
    pub bump: u8,               // store bump seed for this config account
    pub bump_mint_account: u8,  // store bump seed for the stablecoin mint account PDA
}

pub fn process_deposit_collateral_and_mint_tokens(
    ctx: Context<DepositCollateralAndMintTokens>,
    amount_collateral: u64,
    amount_to_mint: u64,
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports() + amount_collateral;
    collateral_account.amount_minted += amount_to_mint;

    if !collateral_account.is_initialized {
        collateral_account.is_initialized = true;
        collateral_account.depositor = ctx.accounts.depositor.key();
        collateral_account.sol_account = ctx.accounts.sol_account.key();
        collateral_account.token_account = ctx.accounts.token_account.key();
        collateral_account.bump = ctx.bumps.collateral_account;
        collateral_account.bump_sol_account = ctx.bumps.sol_account;
    }

    check_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    )?;

    deposit_sol_internal(
        &ctx.accounts.depositor,
        &ctx.accounts.sol_account,
        &ctx.accounts.system_program,
        amount_collateral,
    )?;

    mint_tokens_internal(
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        &ctx.accounts.token_program,
        ctx.accounts.config_account.bump_mint_account,
        amount_to_mint,
    )?;
    Ok(())
}
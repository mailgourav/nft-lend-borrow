///supposedly loan offered has not been taken by a borrower yet. In that case, lender has the opportunity to take out their offer 

pub use anchor_lang::prelude::*;

use crate::states::{CollectionPool, Offer, Vault};

use crate::errors::ErrorCodes;

#[derive(Accounts)]
pub struct WithdrawOffer<'info> {
    #[account(
        mut,
        close = lender,
    )]
    pub offer_loan: Box<Account<'info, Offer>>,

    #[account(
        mut,
        close = lender  /// we pass the close=lender constraint which denotes that we wish to close the offer loan account we pass and the authority to sign the transaction is the lender. Closing an account means that the data of that account will be cleared and the rent be returned to the authority.
    )]  
    pub vault_account: Account<'info, Vault>,    ///it holds the lender SOL and we intent to close the collection_pool, the lender, and the system_program

    #[account(
        mut
    )]
    pub collection_pool: Box<Account<'info, CollectionPool>>,

    #[account(mut)]
    pub lender: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<WithdrawOffer>,
    minimum_balance_for_rent_exemption: u64,        /// solana accounts need to maintain a min balance for them be rent expempt, so that the data in them does not get cleared out. This parameter is measure of that balance for our offer account
) -> Result<()> {
    let collection = &mut ctx.accounts.collection_pool;

    if ctx.accounts.offer_loan.is_loan_taken == true {
        return Err(ErrorCodes::LoanAlreadyTaken.into());
    }
    /// Then, we reduce the number of total offers, since it is being withdrawn. We calculate the amount of lamports the vault has, and then the transfer amount, which will be the vault lamports amount minus the minimum_balance_for_rent_exemption.
    collection.total_offers -= 1;

    let vault_lamports_initial: u64 = ctx.accounts.vault_account.to_account_info().lamports();

    let transfer_amount = vault_lamports_initial
        .checked_sub(minimum_balance_for_rent_exemption)
        .unwrap();

    **ctx.accounts.vault_account.to_account_info().try_borrow_mut_lamports()? -= transfer_amount;

    let mut lamports_ref = ctx.accounts.lender.try_borrow_mut_lamports()?;
    **lamports_ref += transfer_amount;

    Ok(())
}
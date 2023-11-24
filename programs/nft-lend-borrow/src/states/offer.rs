///This is the account which will be created when someone makes an offer.

use anchor_lang::prelude::*;

#[account]
pub struct Offer {
    /// Collection : The address of the collection pool for which the offer is being made
    pub collection: Pubkey,

    /// Offer Amount : The amount of lamports being offered by the lender
    pub offer_lamport_amount: u64,

    /// Repay Amount : The amount of lamports the borrower would have to repay once the loan has been taken.
    pub repay_lamport_amount: u64,

    /// Lender :  The public key of the lender
    pub lender: Pubkey,

    /// Loan Taken : A boolean which keeps track if the loan has been taken or not
    pub is_loan_taken: bool,

    /// Borrower : This will store the borrower pubkey once the loan has been taken
    pub borrower: Pubkey,

    /// Bump : The bump for the offer PDA.
    pub bump: u8,
}

impl Offer {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 32 + 1 + 32 + 1;
}

use anchor_lang::prelude::*;

#[account]
pub struct ActiveLoan {
    /// Collection : The public key of the collection pool account the loan has been taken from.
    pub collection: Pubkey,

    /// Offer Account : The public key of the offer account the borrower has borrowed from.
    pub offer_account: Pubkey,

    /// Lender : The public key of the lender.
    pub lender: Pubkey,

    /// Borrower : The public key of the borrower.
    pub borrower: Pubkey,

    /// NFT Mint : The public key of the NFT mint.
    pub mint: Pubkey,

    /// Loan Taken Timestamp : The unix timestamp when the loan was taken
    pub loan_ts: i64,

    /// Repayment Timestamp : The unix timestamp before which the loan needs to be repaid
    pub repay_ts: i64,

    /// Repaid : A boolean which tracks if the loan is repaid or not
    pub is_repaid: bool,

    /// Liquidated : A boolean which tracks if the loan is liquidated or not
    pub is_liquidated: bool,

    /// Bump : The bump for the active_loan PDA.
    pub bump: u8,
}

impl ActiveLoan {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 32 + 32 + 8 + 8 + 1 + 1 + 1;
}
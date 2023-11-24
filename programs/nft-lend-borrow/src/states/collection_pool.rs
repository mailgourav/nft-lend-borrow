use anchor_lang::prelude::*;

#[account]
pub struct CollectionPool {
    /// The Metaplex Collection ID of the NFT collection
    pub collection_id: Pubkey,

    /// The authority who created the pool
    pub pool_owner: Pubkey,

    /// The amount of time (in unix timestamp) that the loan will stay active
    pub duration: i64,

    /// The total number of offers than have been made.
    pub total_offers: u64,

    /// The bump for the collection pool’s PDA seeds
    pub bump: u8,
}

impl CollectionPool {
    /// number of bytes required to store a collection pool account. 
    /// At first we add 8 bytes which is for the Discriminator. This is required by anchor which allows it to create a unique identification for that account.Collection ID and poolOwner are of pubKey type, which needs 32 bytes.Duration is an i64 type which needs 8 bytes.Total offers is u64 type which also needs 8 bytes. And finally bump, which is of type u8 needs 1 byte.
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 1;
}
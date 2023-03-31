#[cfg(feature = "metadata")]
pub mod token_metadata;

pub mod prelude {
    pub use anchor_lang::solana_program::account_info::AccountInfo;
    pub use anchor_lang::solana_program::program_pack::Pack;
    pub use anchor_lang::solana_program::pubkey::Pubkey;
    pub use anchor_lang::{context::CpiContext, Accounts};
    pub use anchor_lang::{solana_program, Result};
    pub use borsh::{BorshDeserialize, BorshSerialize};
}

use std::ops::Deref;

pub use mpl_token_metadata::{state, ID};

use crate::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn create_metadata_account_v2<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, CreateMetadataAccountV2<'info>>,
    name: String,
    symbol: String,
    uri: String,
    creators: Option<Vec<state::Creator>>,
    seller_fee_basis_points: u16,
    update_authority_is_signer: bool,
    is_mutable: bool,
    collection: Option<state::Collection>,
    uses: Option<state::Uses>,
) -> Result<()> {
    let ix = mpl_token_metadata::instruction::create_metadata_accounts_v2(
        ID,
        *ctx.accounts.metadata.key,
        *ctx.accounts.mint.key,
        *ctx.accounts.mint_authority.key,
        *ctx.accounts.payer.key,
        *ctx.accounts.update_authority.key,
        name,
        symbol,
        uri,
        creators,
        seller_fee_basis_points,
        update_authority_is_signer,
        is_mutable,
        collection,
        uses,
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.metadata.clone(),
            ctx.accounts.mint.clone(),
            ctx.accounts.mint_authority.clone(),
            ctx.accounts.payer.clone(),
            ctx.accounts.update_authority.clone(),
            ctx.accounts.system_program.clone(),
            ctx.accounts.rent.clone(),
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn update_metadata_accounts_v2<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, UpdateMetadataAccountsV2<'info>>,
    new_update_authority: Option<Pubkey>,
    data: Option<state::DataV2>,
    primary_sale_happened: Option<bool>,
    is_mutable: Option<bool>,
) -> Result<()> {
    let ix = mpl_token_metadata::instruction::update_metadata_accounts_v2(
        ID,
        *ctx.accounts.metadata.key,
        *ctx.accounts.update_authority.key,
        new_update_authority,
        data,
        primary_sale_happened,
        is_mutable,
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.metadata.clone(),
            ctx.accounts.update_authority.clone(),
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn update_primary_sale_happened_via_token<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, UpdatePrimarySaleHappenedViaToken<'info>>,
) -> Result<()> {
    let ix = mpl_token_metadata::instruction::update_primary_sale_happened_via_token(
        ID,
        *ctx.accounts.metadata.key,
        *ctx.accounts.owner.key,
        *ctx.accounts.token.key,
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.metadata.clone(),
            ctx.accounts.owner.clone(),
            ctx.accounts.token.clone(),
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn sign_metadata<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SignMetadata<'info>>,
) -> Result<()> {
    let ix = mpl_token_metadata::instruction::sign_metadata(
        ID,
        *ctx.accounts.metadata.key,
        *ctx.accounts.creator.key,
    );
    solana_program::program::invoke_signed(
        &ix,
        &[ctx.accounts.metadata.clone(), ctx.accounts.creator.clone()],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn create_master_edition_v3<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, CreateMasterEditionV3<'info>>,
    max_supply: Option<u64>,
) -> Result<()> {
    let ix = mpl_token_metadata::instruction::create_master_edition_v3(
        ID,
        *ctx.accounts.edition.key,
        *ctx.accounts.mint.key,
        *ctx.accounts.update_authority.key,
        *ctx.accounts.mint_authority.key,
        *ctx.accounts.metadata.key,
        *ctx.accounts.payer.key,
        max_supply,
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.edition.clone(),
            ctx.accounts.mint.clone(),
            ctx.accounts.update_authority.clone(),
            ctx.accounts.mint_authority.clone(),
            ctx.accounts.metadata.clone(),
            ctx.accounts.payer.clone(),
            ctx.accounts.system_program.clone(),
            ctx.accounts.rent.clone(),
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn verify_collection<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, VerifyCollection<'info>>,
) -> Result<()> {
    let ix = mpl_token_metadata::instruction::verify_collection(
        ID,
        *ctx.accounts.metadata.key,
        *ctx.accounts.collection_authority.key,
        *ctx.accounts.payer.key,
        *ctx.accounts.collection_mint.key,
        *ctx.accounts.collection.key,
        *ctx.accounts.collection_master_edition_account.key,
        if ctx.remaining_accounts.is_empty() {
            None
        } else {
            Some(*ctx.remaining_accounts[0].key)
        },
    );
    let mut ixs = vec![
        ctx.accounts.metadata.clone(),
        ctx.accounts.collection_authority.clone(),
        ctx.accounts.payer.clone(),
        ctx.accounts.collection_mint.clone(),
        ctx.accounts.collection.clone(),
        ctx.accounts.collection_master_edition_account.clone(),
    ];
    if !ctx.remaining_accounts.is_empty() {
        ixs.push(ctx.remaining_accounts[0].clone());
    }
    solana_program::program::invoke_signed(&ix, &ixs, ctx.signer_seeds).map_err(Into::into)
}

pub fn approve_collection_authority<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, ApproveCollectionAuthority<'info>>,
) -> Result<()> {
    let ix = mpl_token_metadata::instruction::approve_collection_authority(
        ID,
        *ctx.accounts.collection_authority_record.key,
        *ctx.accounts.new_collection_authority.key,
        *ctx.accounts.update_authority.key,
        *ctx.accounts.payer.key,
        *ctx.accounts.metadata.key,
        *ctx.accounts.mint.key,
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.collection_authority_record.clone(),
            ctx.accounts.new_collection_authority.clone(),
            ctx.accounts.update_authority.clone(),
            ctx.accounts.payer.clone(),
            ctx.accounts.metadata.clone(),
            ctx.accounts.mint.clone(),
            ctx.accounts.token_program.clone(),
            ctx.accounts.system_program.clone(),
            ctx.accounts.rent.clone(),
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct UpdatePrimarySaleHappenedViaToken<'info> {
    /// CHECK:
    pub metadata: AccountInfo<'info>,
    /// CHECK:
    pub owner: AccountInfo<'info>,
    /// CHECK:
    pub token: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateMetadataAccountV2<'info> {
    /// CHECK:
    pub metadata: AccountInfo<'info>,
    /// CHECK:
    pub mint: AccountInfo<'info>,
    /// CHECK:
    pub mint_authority: AccountInfo<'info>,
    /// CHECK:
    pub payer: AccountInfo<'info>,
    /// CHECK:
    pub update_authority: AccountInfo<'info>,
    /// CHECK:
    pub system_program: AccountInfo<'info>,
    /// CHECK:
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateMasterEditionV3<'info> {
    /// CHECK:
    pub edition: AccountInfo<'info>,
    /// CHECK:
    pub mint: AccountInfo<'info>,
    /// CHECK:
    pub update_authority: AccountInfo<'info>,
    /// CHECK:
    pub mint_authority: AccountInfo<'info>,
    /// CHECK:
    pub payer: AccountInfo<'info>,
    /// CHECK:
    pub metadata: AccountInfo<'info>,
    /// CHECK:
    pub token_program: AccountInfo<'info>,
    /// CHECK:
    pub system_program: AccountInfo<'info>,
    /// CHECK:
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateMetadataAccountsV2<'info> {
    /// CHECK: mutable
    pub metadata: AccountInfo<'info>,
    /// CHECK: signer
    pub update_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SignMetadata<'info> {
    /// CHECK: mutable
    pub metadata: AccountInfo<'info>,
    /// CHECK: signer
    pub creator: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct VerifyCollection<'info> {
    /// CHECK: metadata account
    pub metadata: AccountInfo<'info>,
    /// CHECK: collection update authority
    pub collection_authority: AccountInfo<'info>,
    /// CHECK:
    pub payer: AccountInfo<'info>,
    /// CHECK: mint of collection
    pub collection_mint: AccountInfo<'info>,
    /// CHECK: metadata account of collection
    pub collection: AccountInfo<'info>,
    /// CHECK: master edition v2 account of collection token
    pub collection_master_edition_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ApproveCollectionAuthority<'info> {
    /// CHECK: empty account which derived from ['metadata', metadata::ID, mint, 'collection_authority', new_collection_authority]
    pub collection_authority_record: AccountInfo<'info>,
    /// CHECK:
    pub new_collection_authority: AccountInfo<'info>,
    /// CHECK:
    pub update_authority: AccountInfo<'info>,
    /// CHECK:
    pub payer: AccountInfo<'info>,
    /// CHECK:
    pub metadata: AccountInfo<'info>,
    /// CHECK:
    pub mint: AccountInfo<'info>,
    /// CHECK:
    pub token_program: AccountInfo<'info>,
    /// CHECK:
    pub system_program: AccountInfo<'info>,
    /// CHECK:
    pub rent: AccountInfo<'info>,
}

#[derive(Debug, Clone)]
pub struct MetadataAccount(state::Metadata);

impl MetadataAccount {
    pub const LEN: usize = state::MAX_METADATA_LEN;
}

impl anchor_lang::AccountSerialize for MetadataAccount {}

impl anchor_lang::AccountDeserialize for MetadataAccount {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(MetadataAccount(state::Metadata::deserialize(buf)?))
    }
}

impl anchor_lang::Owner for MetadataAccount {
    fn owner() -> Pubkey {
        ID
    }
}

impl Deref for MetadataAccount {
    type Target = state::Metadata;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct MasterEditionAccount(state::MasterEditionV2);

impl MasterEditionAccount {
    pub const LEN: usize = state::MAX_MASTER_EDITION_LEN;
}

impl anchor_lang::AccountSerialize for MasterEditionAccount {}

impl anchor_lang::AccountDeserialize for MasterEditionAccount {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(MasterEditionAccount(state::MasterEditionV2::deserialize(
            buf,
        )?))
    }
}

impl anchor_lang::Owner for MasterEditionAccount {
    fn owner() -> Pubkey {
        ID
    }
}

impl Deref for MasterEditionAccount {
    type Target = state::MasterEditionV2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct CollectionAuthorityRecord(state::CollectionAuthorityRecord);

impl CollectionAuthorityRecord {
    pub const LEN: usize = state::COLLECTION_AUTHORITY_RECORD_SIZE;
}

impl anchor_lang::AccountSerialize for CollectionAuthorityRecord {}

impl anchor_lang::AccountDeserialize for CollectionAuthorityRecord {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(CollectionAuthorityRecord(
            state::CollectionAuthorityRecord::deserialize(buf)?,
        ))
    }
}

impl anchor_lang::Owner for CollectionAuthorityRecord {
    fn owner() -> Pubkey {
        ID
    }
}

impl Deref for CollectionAuthorityRecord {
    type Target = state::CollectionAuthorityRecord;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct TokenMetadata;

impl anchor_lang::Id for TokenMetadata {
    fn id() -> Pubkey {
        ID
    }
}

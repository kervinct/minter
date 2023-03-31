use std::ops::{Deref, DerefMut};

use anchor_lang::prelude::*;
use anchor_metadata::token_metadata::{self, TokenMetadata};
use anchor_spl::associated_token::{self, AssociatedToken};
use anchor_spl::token::{self, Mint, Token};
use solana_program::{
    program::{invoke, invoke_signed},
    system_instruction,
};

solana_security_txt::security_txt! {
    name: "Minter",
    project_url: "",
    contacts: "",
    policy: ""
}

declare_id!("D2bEcGfANjxMf9ZxkVVvbtC6tsoo2YVfnT8nP6TTRBWJ");

#[error_code]
pub enum MinterError {
    #[msg("Invalid mint address")]
    InvalidMintAddress,
    #[msg("Invalid associated token address")]
    InvalidAssociatedTokenAddress,
    #[msg("Invalid metadata address")]
    InvalidTokenMetadataAddress,
    #[msg("Invalid master edition address")]
    InvalidMasterEditionAddress,
    #[msg("Mismatched minter owner")]
    MismatchedMinterOwner,
}

const MINTER_KEY: &[u8] = b"minter";
const MINTER_KEY_V2: &[u8] = b"minter_v2";
const MINT_KEY: &[u8] = b"mint";
const MINT_KEY_V2: &[u8] = b"mint_v2";

#[program]
pub mod minter {
    use super::*;

    pub fn initialize_minter(ctx: Context<InitializeMinter>) -> Result<()> {
        let minter = &mut ctx.accounts.minter;
        minter.owner = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn initialize_minter_v2(ctx: Context<InitializeMinterV2>) -> Result<()> {
        let minter = &mut ctx.accounts.minter;
        minter.owner = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn binding_collection(ctx: Context<BindingCollection>) -> Result<()> {
        let minter_seeds = &[MINTER_KEY_V2, ctx.accounts.minter.owner.as_ref()];
        let (_pda, bump_seed) = Pubkey::find_program_address(&minter_seeds[..], ctx.program_id);
        let minter_seeds = &[
            MINTER_KEY_V2,
            ctx.accounts.minter.owner.as_ref(),
            &[bump_seed],
        ];

        token_metadata::approve_collection_authority(
            ctx.accounts
                .into_approve_collection_authority_context()
                .with_signer(&[&minter_seeds[..]]),
        )?;

        let minter = &mut ctx.accounts.minter;
        minter.collection = Some(ctx.accounts.mint.key());

        Ok(())
    }

    pub fn mint_for(
        ctx: Context<MintFor>,
        name: String,
        symbol: String,
        uri: String,
        seller_fee_basis_points: u16,
        is_mutable: bool,
    ) -> Result<()> {
        let minter = &ctx.accounts.minter;
        let minter_count_bytes = minter.count.to_le_bytes();

        let minter_seeds = &[MINTER_KEY, ctx.accounts.minter.owner.as_ref()];
        let (_pda, bump_seed) = Pubkey::find_program_address(&minter_seeds[..], ctx.program_id);
        let minter_seeds = &[MINTER_KEY, ctx.accounts.minter.owner.as_ref(), &[bump_seed]];

        // check that mint address is a valid program derived address
        let mint_seeds = &[MINT_KEY, &minter_count_bytes[..]];
        let (mint_addr, mint_seed) = Pubkey::find_program_address(&mint_seeds[..], ctx.program_id);
        if ctx.accounts.mint.key != &mint_addr {
            return Err(MinterError::InvalidMintAddress.into());
        }
        let mint_seeds = &[MINT_KEY, &minter_count_bytes[..], &[mint_seed]];
        allocate_mint_for(&ctx, &[&mint_seeds[..]])?;

        token::initialize_mint(
            ctx.accounts.into_initialize_mint_context(),
            0,
            &ctx.accounts.minter.key(),
            None,
        )?;

        associated_token::create(
            ctx.accounts
                .into_create_associated_token_context(&ctx.accounts.payer),
        )?;

        token::mint_to(
            ctx.accounts
                .into_token_mint_to_context(&ctx.accounts.minter.to_account_info())
                .with_signer(&[&minter_seeds[..]]),
            1,
        )?;

        token_metadata::create_metadata_account_v2(
            ctx.accounts
                .into_create_metadata_context(
                    &ctx.accounts.payer,
                    &ctx.accounts.minter.to_account_info(),
                )
                .with_signer(&[&minter_seeds[..]]),
            name,
            symbol,
            uri,
            Some(vec![
                token_metadata::state::Creator {
                    address: *ctx.accounts.payer.key,
                    verified: false,
                    share: 98,
                },
                token_metadata::state::Creator {
                    address: ctx.accounts.minter.key(),
                    verified: true,
                    share: 2,
                },
            ]),
            seller_fee_basis_points,
            true,
            is_mutable,
            None,
            None,
        )?;

        token_metadata::create_master_edition_v3(
            ctx.accounts
                .into_create_master_edition_context(
                    &ctx.accounts.payer,
                    &ctx.accounts.minter.to_account_info(),
                )
                .with_signer(&[&minter_seeds[..]]),
            Some(0),
        )?;

        token_metadata::sign_metadata(
            ctx.accounts.into_sign_metadata_context(&ctx.accounts.payer),
        )?;

        ctx.accounts.minter.reload()?;
        let minter = &mut ctx.accounts.minter;
        minter.count += 1;

        Ok(())
    }

    pub fn mint_with_collection(
        ctx: Context<MintWithCollection>,
        name: String,
        symbol: String,
        uri: String,
        seller_fee_basis_points: u16,
        is_mutable: bool,
    ) -> Result<()> {
        let minter = &ctx.accounts.minter;
        let minter_count_bytes = minter.count.to_le_bytes();

        let minter_seeds = &[MINTER_KEY_V2, ctx.accounts.minter.owner.as_ref()];
        let (_pda, bump_seed) = Pubkey::find_program_address(&minter_seeds[..], ctx.program_id);
        let minter_seeds = &[
            MINTER_KEY_V2,
            ctx.accounts.minter.owner.as_ref(),
            &[bump_seed],
        ];

        // check that mint address is a valid program derived address
        let mint_seeds = &[MINT_KEY_V2, &minter_count_bytes[..]];
        let (mint_addr, mint_seed) = Pubkey::find_program_address(&mint_seeds[..], ctx.program_id);
        if ctx.accounts.mint.key != &mint_addr {
            return Err(MinterError::InvalidMintAddress.into());
        }
        let mint_seeds = &[MINT_KEY_V2, &minter_count_bytes[..], &[mint_seed]];
        allocate_mint_with_collection(&ctx, &[&mint_seeds[..]])?;

        token::initialize_mint(
            ctx.accounts.into_initialize_mint_context(),
            0,
            &ctx.accounts.minter.key(),
            None,
        )?;

        associated_token::create(
            ctx.accounts
                .into_create_associated_token_context(&ctx.accounts.payer),
        )?;

        token::mint_to(
            ctx.accounts
                .into_token_mint_to_context(&ctx.accounts.minter.to_account_info())
                .with_signer(&[&minter_seeds[..]]),
            1,
        )?;

        token_metadata::create_metadata_account_v2(
            ctx.accounts
                .into_create_metadata_context(
                    &ctx.accounts.payer,
                    &ctx.accounts.minter.to_account_info(),
                )
                .with_signer(&[&minter_seeds[..]]),
            name,
            symbol,
            uri,
            Some(vec![
                token_metadata::state::Creator {
                    address: *ctx.accounts.payer.key,
                    verified: false,
                    share: 98,
                },
                token_metadata::state::Creator {
                    address: ctx.accounts.minter.key(),
                    verified: true,
                    share: 2,
                },
            ]),
            seller_fee_basis_points,
            true,
            is_mutable,
            minter
                .collection
                .map(|mint| token_metadata::state::Collection {
                    key: mint,
                    verified: false,
                }),
            None,
        )?;

        token_metadata::create_master_edition_v3(
            ctx.accounts
                .into_create_master_edition_context(
                    &ctx.accounts.payer,
                    &ctx.accounts.minter.to_account_info(),
                )
                .with_signer(&[&minter_seeds[..]]),
            Some(0),
        )?;

        token_metadata::sign_metadata(
            ctx.accounts.into_sign_metadata_context(&ctx.accounts.payer),
        )?;

        token_metadata::verify_collection(
            ctx.accounts
                .into_verify_collection_context()
                .with_remaining_accounts(vec![ctx
                    .accounts
                    .collection_authority_record
                    .to_account_info()
                    .clone()])
                .with_signer(&[&minter_seeds[..]]),
        )?;

        ctx.accounts.minter.reload()?;
        let minter = &mut ctx.accounts.minter;
        minter.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMinter<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init,
        seeds = [b"minter", authority.key.as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + 4,
    )]
    pub minter: Account<'info, MinterAccount>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitializeMinterV2<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init,
        seeds = [b"minter_v2", authority.key.as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + 4 + 33,
    )]
    pub minter: Account<'info, MinterAccountV2>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(Default, Debug)]
#[repr(C)]
pub struct MinterAccountV2 {
    pub owner: Pubkey,
    pub count: u32,
    pub collection: Option<Pubkey>,
}

#[account]
#[derive(Default, Debug)]
#[repr(C)]
pub struct MinterAccount {
    pub owner: Pubkey,
    pub count: u32,
}

#[derive(Accounts)]
pub struct BindingCollection<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: ['metadata', metadata::ID, mint, 'collection_authority', new_collection_authority]
    #[account(mut)]
    pub collection_authority_record: UncheckedAccount<'info>,

    #[account(mut,
        constraint = &minter.owner == authority.key @ MinterError::MismatchedMinterOwner,
    )]
    pub minter: Account<'info, MinterAccountV2>,

    #[account(
        constraint = &metadata.update_authority == authority.key,
    )]
    pub metadata: Account<'info, token_metadata::MetadataAccount>,

    pub mint: Account<'info, token::Mint>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, token_metadata::TokenMetadata>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> BindingCollection<'info> {
    pub fn into_approve_collection_authority_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token_metadata::ApproveCollectionAuthority<'info>> {
        let cpi_program = self.token_metadata_program.to_account_info();
        let cpi_accounts = token_metadata::ApproveCollectionAuthority {
            collection_authority_record: self.collection_authority_record.to_account_info(),
            new_collection_authority: self.minter.to_account_info(),
            update_authority: self.authority.to_account_info(),
            payer: self.authority.to_account_info(),
            metadata: self.metadata.to_account_info(),
            mint: self.mint.to_account_info(),
            token_program: self.token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct BasicMint<'info> {
    /// CHECK: ['mint', count.to_le_bytes()] or ['mint_v2', count.to_le_bytes()]
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,

    /// CHECK: [payer, token::ID, mint]
    #[account(mut)]
    pub associated_token_account: UncheckedAccount<'info>,

    /// CHECK: ['metadata', metadata::ID, mint]
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: ['metadata', metadata::ID, mint, 'edition']
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, TokenMetadata>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> BasicMint<'info> {
    pub fn into_initialize_mint_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token::InitializeMint<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::InitializeMint {
            mint: self.mint.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_create_associated_token_context(
        &self,
        payer: &Signer<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, associated_token::Create<'info>> {
        let cpi_program = self.associated_token_program.to_account_info();
        let cpi_accounts = associated_token::Create {
            payer: payer.to_account_info(),
            associated_token: self.associated_token_account.to_account_info(),
            authority: payer.to_account_info(),
            mint: self.mint.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_token_mint_to_context(
        &self,
        minter: &AccountInfo<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, token::MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::MintTo {
            mint: self.mint.to_account_info(),
            to: self.associated_token_account.to_account_info(),
            authority: minter.clone(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_create_metadata_context(
        &self,
        payer: &Signer<'info>,
        minter: &AccountInfo<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, token_metadata::CreateMetadataAccountV2<'info>> {
        let cpi_program = self.token_metadata_program.to_account_info();
        let cpi_accounts = token_metadata::CreateMetadataAccountV2 {
            metadata: self.metadata.to_account_info(),
            mint: self.mint.to_account_info(),
            mint_authority: minter.clone(),
            payer: payer.to_account_info(),
            update_authority: minter.clone(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_create_master_edition_context(
        &self,
        payer: &Signer<'info>,
        minter: &AccountInfo<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, token_metadata::CreateMasterEditionV3<'info>> {
        let cpi_program = self.token_metadata_program.to_account_info();
        let cpi_accounts = token_metadata::CreateMasterEditionV3 {
            edition: self.master_edition.to_account_info(),
            mint: self.mint.to_account_info(),
            update_authority: minter.clone(),
            mint_authority: minter.clone(),
            payer: payer.to_account_info(),
            metadata: self.metadata.to_account_info(),
            token_program: self.token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn into_sign_metadata_context(
        &self,
        payer: &Signer<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, token_metadata::SignMetadata<'info>> {
        let cpi_program = self.token_metadata_program.to_account_info();
        let cpi_accounts = token_metadata::SignMetadata {
            metadata: self.metadata.to_account_info(),
            creator: payer.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct MintFor<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub minter: Account<'info, MinterAccount>,

    pub basic: BasicMint<'info>,
}

impl<'info> Deref for MintFor<'info> {
    type Target = BasicMint<'info>;

    fn deref(&self) -> &Self::Target {
        &self.basic
    }
}

impl<'info> DerefMut for MintFor<'info> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.basic
    }
}

#[derive(Accounts)]
pub struct MintWithCollection<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub minter: Account<'info, MinterAccountV2>,

    #[account(
        seeds = [b"metadata", basic.token_metadata_program.key.as_ref(), collection_mint.key().as_ref(), b"collection_authority", minter.key().as_ref()],
        bump,
        seeds::program = basic.token_metadata_program.key,
    )]
    pub collection_authority_record: Box<Account<'info, token_metadata::CollectionAuthorityRecord>>,

    pub collection_mint: Box<Account<'info, token::Mint>>,
    pub collection: Box<Account<'info, token_metadata::MetadataAccount>>,
    pub collection_master_edition_account:
        Box<Account<'info, token_metadata::MasterEditionAccount>>,

    pub basic: BasicMint<'info>,
}

impl<'info> MintWithCollection<'info> {
    pub fn into_verify_collection_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, token_metadata::VerifyCollection<'info>> {
        let cpi_program = self.token_metadata_program.to_account_info();
        let cpi_accounts = token_metadata::VerifyCollection {
            metadata: self.metadata.to_account_info(),
            collection_authority: self.minter.to_account_info(),
            payer: self.payer.to_account_info(),
            collection_mint: self.collection_mint.to_account_info(),
            collection: self.collection.to_account_info(),
            collection_master_edition_account: self
                .collection_master_edition_account
                .to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> Deref for MintWithCollection<'info> {
    type Target = BasicMint<'info>;

    fn deref(&self) -> &Self::Target {
        &self.basic
    }
}

impl<'info> DerefMut for MintWithCollection<'info> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.basic
    }
}

#[inline(always)]
pub fn allocate_mint_for(ctx: &Context<MintFor>, signer_seeds: &[&[&[u8]]]) -> Result<()> {
    create_or_allocate_account_raw(
        *ctx.accounts.token_program.key,
        &ctx.accounts.mint.to_account_info(),
        &ctx.accounts.rent.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        &ctx.accounts.payer.to_account_info(),
        Mint::LEN,
        signer_seeds,
    )
}

#[inline(always)]
pub fn allocate_mint_with_collection(
    ctx: &Context<MintWithCollection>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    create_or_allocate_account_raw(
        *ctx.accounts.token_program.key,
        &ctx.accounts.mint.to_account_info(),
        &ctx.accounts.rent.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        &ctx.accounts.payer.to_account_info(),
        Mint::LEN,
        signer_seeds,
    )
}

#[inline(always)]
pub fn create_or_allocate_account_raw<'a>(
    program_id: Pubkey,
    new_account_info: &AccountInfo<'a>,
    rent_sysvar_info: &AccountInfo<'a>,
    system_program_info: &AccountInfo<'a>,
    payer_info: &AccountInfo<'a>,
    size: usize,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let current_lamports = new_account_info.lamports();
    let rent = &Rent::from_account_info(rent_sysvar_info)?;
    if current_lamports == 0 {
        let lamports = rent.minimum_balance(size);
        invoke_signed(
            &system_instruction::create_account(
                payer_info.key,
                new_account_info.key,
                lamports,
                size.try_into().unwrap(),
                &program_id,
            ),
            &[payer_info.clone(), new_account_info.clone()],
            signer_seeds,
        )?;
    } else {
        let required_lamports = rent
            .minimum_balance(size)
            .max(1)
            .saturating_sub(current_lamports);

        if required_lamports > 0 {
            // transfer lamports from payer to new account
            invoke(
                &system_instruction::transfer(
                    payer_info.key,
                    new_account_info.key,
                    required_lamports,
                ),
                &[
                    payer_info.clone(),
                    new_account_info.clone(),
                    system_program_info.clone(),
                ],
                // signer_seeds,
            )?;
        }

        let accounts = &[new_account_info.clone(), system_program_info.clone()];

        // allocate space
        invoke_signed(
            &system_instruction::allocate(new_account_info.key, size.try_into().unwrap()),
            accounts,
            signer_seeds,
        )?;

        // assign owner from system program to specified program
        invoke_signed(
            &system_instruction::assign(new_account_info.key, &program_id),
            accounts,
            signer_seeds,
        )?;
    }

    Ok(())
}

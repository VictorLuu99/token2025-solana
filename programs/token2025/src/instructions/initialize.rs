use anchor_lang::prelude::*;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata as Metaplex},
    token::{
        mint_to, set_authority, spl_token::instruction::AuthorityType, Mint, MintTo, SetAuthority,
        Token, TokenAccount,
    },
};
use mpl_token_metadata::types::DataV2;

#[derive(Accounts)]
#[instruction(name: String, symbol: String, uri: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        mint::decimals = 9,
        mint::authority = admin,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = mint,
        associated_token::authority = admin,
    )]
    pub admin_token_account: Account<'info, TokenAccount>,

    /// CHECK: Metaplex will check this
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metaplex>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize(
    ctx: Context<Initialize>,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    let token_data = DataV2 {
        name: name.clone(),
        symbol: symbol.clone(),
        uri: uri.clone(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let metadata_ctx = CpiContext::new(
        ctx.accounts.token_metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
            metadata: ctx.accounts.metadata.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            mint_authority: ctx.accounts.admin.to_account_info(),
            payer: ctx.accounts.admin.to_account_info(),
            update_authority: ctx.accounts.admin.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
    );

    create_metadata_accounts_v3(
        metadata_ctx,
        token_data,
        true,  // is_mutable
        true,  // update_authority_is_signer
        None,  // collection_details
    )?;

    // Mint initial supply to admin
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.admin_token_account.to_account_info(),
                authority: ctx.accounts.admin.to_account_info(),
            },
            &[],
        ),
        1_000_000 * 10u64.pow(9), // 1 million tokens with 9 decimals
    )?;

    Ok(())
}
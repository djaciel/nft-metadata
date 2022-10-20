use {
    anchor_lang::{prelude::*, system_program},
    anchor_spl::{associated_token, token},
    utils::{MetaplexKey, PREFIX, Metadata}
};

pub mod utils;
use utils::*;

declare_id!("BPnzXRAP6GejnumGTZp1fJzUWX5veRCgN7cJ3QAPssLv");

#[program]
pub mod nft_metadata {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn mint(
        ctx: Context<MintMetaplexNft>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
    ) -> Result<()> {
        msg!("Creating mint account...");
        msg!("Mint: {}", &ctx.accounts.mint.key());
        system_program::create_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.mint_authority.to_account_info(),
                    to: ctx.accounts.mint.to_account_info(),
                },
            ),
            10000000,
            82,
            &ctx.accounts.token_program.key(),
        )?;
    
        msg!("Initializing mint account...");
        msg!("Mint: {}", &ctx.accounts.mint.key());
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            0,
            &ctx.accounts.mint_authority.key(),
            Some(&ctx.accounts.mint_authority.key()),
        )?;
    
        msg!("Creating token account...");
        msg!("Token Address: {}", &ctx.accounts.token_account.key());
        associated_token::create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.mint_authority.to_account_info(),
                associated_token: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ))?;
    
        msg!("Minting token to token account...");
        msg!("Mint: {}", &ctx.accounts.mint.to_account_info().key());
        msg!("Token Address: {}", &ctx.accounts.token_account.key());
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            1,
        )?;
    
        msg!("Creating metadata account...");
    
        let metadata = &mut ctx.accounts.metadata;
        metadata.data.name = metadata_title;
        metadata.data.symbol = metadata_symbol;
        metadata.data.uri = metadata_uri;
        metadata.data.seller_fee_basis_points = 1;
    
        metadata.key = MetaplexKey::MetadataV1;
        metadata.primary_sale_happened = false;
        metadata.is_mutable = true;
        // without edition
    
        msg!("Token mint process completed successfully.");
    
        Ok(())
    }
}


#[derive(Accounts)]
pub struct MintMetaplexNft<'info> {
    #[account(
        init,
        payer = mint_authority,
        space = 387,
        seeds = [
            PREFIX.as_ref(),
            token_metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump
    )]
    pub metadata: Account<'info, Metadata>,
    #[account(mut)]
    pub mint: Signer<'info>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Initialize {}

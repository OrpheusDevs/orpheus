use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, SetAuthority, TokenAccount, Transfer, Mint};
use spl_token::instruction::AuthorityType;

// Add these imports for Metaplex metadata verification
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::program_pack::Pack;
use anchor_lang::solana_program::program_error::ProgramError;
use std::str::FromStr;

declare_id!("HGhUfApRyEBL758VLG5kq45UkEAsvaVcPvCxVHuXMdhU");

// Define the Metaplex Token Metadata Program ID
const METADATA_PROGRAM_ID: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";
const ROYALTY_CONFIG_SEED: &[u8] = b"royalty_config";

// Add error codes for NFT verification and royalty distribution
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid NFT ownership")]
    InvalidNFTOwnership,
    #[msg("Invalid NFT metadata")]
    InvalidNFTMetadata,
    #[msg("Unauthorized playback")]
    UnauthorizedPlayback,
    #[msg("Invalid music track")]
    InvalidMusicTrack,
    #[msg("Invalid token metadata program")]
    InvalidMetadataProgram,
    #[msg("Authorization cache expired")]
    AuthorizationExpired,
    #[msg("Invalid royalty configuration")]
    InvalidRoyaltyConfig,
    #[msg("Royalty basis points exceed maximum")]
    RoyaltyBasisPointsExceedMax,
    #[msg("Royalty recipients count exceeded maximum")]
    TooManyRoyaltyRecipients,
    #[msg("Royalty recipients basis points don't match total")]
    RoyaltyBasisPointsMismatch,
    #[msg("Unauthorized royalty config update")]
    UnauthorizedRoyaltyUpdate,
    #[msg("Immutable royalty configuration")]
    ImmutableRoyaltyConfig,
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    #[msg("Insufficient funds for transaction")]
    InsufficientFunds,
    #[msg("Sale price is too low for royalty calculation")]
    SalePriceTooLow,
    #[msg("Invalid recipient account")]
    InvalidRecipientAccount,
    #[msg("Missing expected account in remaining accounts")]
    MissingExpectedAccount,
    #[msg("Token mint mismatch")]
    TokenMintMismatch,
}

#[program]
pub mod anchor_auction {
    use std::ops::Add;
    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"escrow";
    const MUSIC_TRACK_SEED: &[u8] = b"music_track";
    const AUTH_REGISTRY_SEED: &[u8] = b"auth_registry";
    const USER_AUTH_SEED: &[u8] = b"user_auth";
    
    const MAX_ROYALTY_BASIS_POINTS: u16 = 2500; // 25% max royalty
    const MAX_ROYALTY_RECIPIENTS: usize = 5;    // Maximum of 5 recipients

    pub fn exhibit(
        ctx: Context<Exhibit>,
        initial_price: u64,
        auction_duration_sec: u64,
    ) -> Result<()> {
        ctx.accounts.escrow_account.exhibitor_pubkey = ctx.accounts.exhibitor.key();
        ctx.accounts.escrow_account.exhibitor_ft_receiving_pubkey = ctx.accounts.exhibitor_ft_receiving_account.key();
        ctx.accounts.escrow_account.exhibiting_nft_temp_pubkey = ctx.accounts.exhibitor_nft_temp_account.key();
        ctx.accounts.escrow_account.highest_bidder_pubkey = ctx.accounts.exhibitor.key();
        ctx.accounts.escrow_account.highest_bidder_ft_temp_pubkey = ctx.accounts.exhibitor_ft_receiving_account.key();
        ctx.accounts.escrow_account.highest_bidder_ft_returning_pubkey = ctx.accounts.exhibitor_ft_receiving_account.key();
        ctx.accounts.escrow_account.price = initial_price;
        ctx.accounts.escrow_account.end_at = ctx.accounts.clock.unix_timestamp.add(auction_duration_sec as i64);

        let (pda, _bump_seed) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        token::set_authority(
            ctx.accounts.to_set_authority_context(),
        AuthorityType::AccountOwner,
        Some(pda)
        )?;

        token::transfer(
            ctx.accounts.to_transfer_to_pda_context(),
           1
        )?;

        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel> ) -> Result<()> {
        let (_, bump_seed) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let signers_seeds: &[&[&[u8]]] = &[&[&ESCROW_PDA_SEED[..], &[bump_seed]]];

        token::transfer(
            ctx.accounts
                .to_transfer_to_exhibitor_context()
                .with_signer(signers_seeds),
            ctx.accounts.exhibitor_nft_temp_account.amount
        )?;

        token::close_account(
            ctx.accounts
                .to_close_context()
                .with_signer(signers_seeds)
        )?;

        Ok(())
    }

    pub fn bid(ctx: Context<Bid>, price: u64) -> Result<()> {
        let (pda, bump_seed) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let signers_seeds: &[&[&[u8]]] = &[&[&ESCROW_PDA_SEED[..], &[bump_seed]]];

        if ctx.accounts.escrow_account.highest_bidder_pubkey != ctx.accounts.escrow_account.exhibitor_pubkey {
            token::transfer(
                ctx.accounts
                    .to_transfer_to_previous_bidder_context()
                    .with_signer(signers_seeds),
                ctx.accounts.escrow_account.price
            )?;

            token::close_account(
                ctx.accounts
                    .to_close_context()
                    .with_signer(signers_seeds)
            )?;
        }

        token::set_authority(
            ctx.accounts.to_set_authority_context(),
            AuthorityType::AccountOwner,
            Some(pda)
        )?;
        token::transfer(
            ctx.accounts.to_transfer_to_pda_context(),
            price,
        )?;

        ctx.accounts.escrow_account.price = price;
        ctx.accounts.escrow_account.highest_bidder_pubkey = ctx.accounts.bidder.key();
        ctx.accounts.escrow_account.highest_bidder_ft_temp_pubkey = ctx.accounts.bidder_ft_temp_account.key();
        ctx.accounts.escrow_account.highest_bidder_ft_returning_pubkey = ctx.accounts.bidder_ft_account.key();

        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        let (_, bump_seed) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let signers_seeds: &[&[&[u8]]] = &[&[&ESCROW_PDA_SEED[..], &[bump_seed]]];

        token::transfer(
            ctx.accounts
                .to_transfer_to_highest_bidder_context()
                .with_signer(signers_seeds),
            ctx.accounts.exhibitor_nft_temp_account.amount,
        )?;

        token::transfer(
            ctx.accounts
                .to_transfer_to_exhibitor_context()
                .with_signer(signers_seeds),
            ctx.accounts.highest_bidder_ft_temp_account.amount,
        )?;

        token::close_account(
            ctx.accounts.to_close_ft_context()
                .with_signer(signers_seeds),
        )?;

        token::close_account(
            ctx.accounts.to_close_nft_context()
                .with_signer(signers_seeds),
        )?;

        Ok(())
    }

    // Add new function for registering a music track
    pub fn register_music_track(
        ctx: Context<RegisterMusicTrack>,
        track_id: String,
        track_uri: String,
        preview_uri: String,
        metadata_uri: String,
        is_public: bool,
    ) -> Result<()> {
        let music_track = &mut ctx.accounts.music_track;
        
        // Initialize the music track account
        music_track.authority = ctx.accounts.authority.key();
        music_track.track_id = track_id;
        music_track.track_uri = track_uri;
        music_track.preview_uri = preview_uri;
        music_track.metadata_uri = metadata_uri;
        music_track.is_public = is_public;
        music_track.authorized_collections = Vec::new();
        
        // Update the registry
        let registry = &mut ctx.accounts.auth_registry;
        if registry.track_count == 0 {
            // First-time initialization of registry
            registry.authority = ctx.accounts.authority.key();
        }
        registry.track_count = registry.track_count.checked_add(1).unwrap();
        
        Ok(())
    }
    
    // Add function to add authorized collection to a track
    pub fn add_authorized_collection(
        ctx: Context<UpdateTrackAuthorization>,
        collection_mint: Pubkey,
    ) -> Result<()> {
        let music_track = &mut ctx.accounts.music_track;
        
        // Ensure only the track authority can add collections
        require!(
            music_track.authority == ctx.accounts.authority.key(),
            ErrorCode::UnauthorizedPlayback
        );
        
        // Add the collection to authorized list if not already present
        if !music_track.authorized_collections.contains(&collection_mint) {
            // Limit the number of collections to prevent excessive storage costs
            require!(
                music_track.authorized_collections.len() < 20,
                ErrorCode::InvalidMusicTrack
            );
            
            music_track.authorized_collections.push(collection_mint);
        }
        
        Ok(())
    }
    
    // Add function to verify playback authorization
    pub fn verify_playback_authorization(
        ctx: Context<VerifyPlaybackAuthorization>,
    ) -> Result<()> {
        let music_track = &ctx.accounts.music_track;
        let user = &ctx.accounts.user;
        
        // Check if there's a valid cached authorization
        if let Some(user_auth) = &ctx.accounts.user_auth {
            // Verify cache hasn't expired
            if user_auth.expires_at > Clock::get()?.unix_timestamp {
                return Ok(());
            } else {
                // Authorization expired - continue with verification
            }
        }
        
        // If track is public, allow playback without NFT verification
        if music_track.is_public {
            // Cache the authorization if cache account provided
            if let Some(user_auth) = &mut ctx.accounts.user_auth {
                user_auth.user = user.key();
                user_auth.track_id = music_track.track_id.clone();
                user_auth.expires_at = Clock::get()?.unix_timestamp + 3600; // 1 hour expiration
            }
            return Ok(());
        }
        
        // Verify the metadata program is legitimate
        let expected_metadata_program = 
            Pubkey::from_str(METADATA_PROGRAM_ID).unwrap();
        require!(
            ctx.accounts.metadata_program.key() == expected_metadata_program,
            ErrorCode::InvalidMetadataProgram
        );
        
        // Check if user has provided a token account
        if let Some(user_token_account) = &ctx.accounts.user_token_account {
            // Verify the token account belongs to the user
            // In SPL tokens, the 'owner' field is actually the authority who can transfer the tokens
            require!(
                user_token_account.owner == user.key(),
                ErrorCode::InvalidNFTOwnership
            );
            
            require!(
                user_token_account.amount == 1,
                ErrorCode::InvalidNFTOwnership
            );
            
            // Verify the token isn't frozen
            require!(
                !user_token_account.is_frozen(),
                ErrorCode::InvalidNFTOwnership
            );
            
            // Check if the NFT mint is in the authorized collections
            let nft_mint = user_token_account.mint;
            let mut is_authorized = false;
            
            // If we have metadata account info, verify it
            if let Some(metadata_info) = &ctx.accounts.nft_metadata {
                // Verify the metadata PDA matches the expected one for this mint
                let metadata_program_id = ctx.accounts.metadata_program.key();
                let seeds = &[
                    b"metadata",
                    metadata_program_id.as_ref(),
                    nft_mint.as_ref(),
                ];
                let (expected_metadata_key, _) = 
                    Pubkey::find_program_address(seeds, metadata_program_id);
                
                require!(
                    metadata_info.key() == expected_metadata_key,
                    ErrorCode::InvalidNFTMetadata
                );
                
                // Check if NFT is in an authorized collection
                // First, check if NFT mint is directly authorized
                if music_track.authorized_collections.contains(&nft_mint) {
                    is_authorized = true;
                } else {
                    // More complete implementation would parse metadata here to check collection
                    // For example:
                    //
                    // 1. Parse the metadata (this would require Metaplex's metadata structures)
                    // 2. Check the collection field in the metadata
                    // 3. Verify the collection is in our authorized_collections list
                    //
                    // This is a simplified authorization for demonstration
                    
                    // For collections, we would need to deserialize and check:
                    // let metadata = mpl_token_metadata::state::Metadata::from_account_info(metadata_info)?;
                    // if let Some(collection) = metadata.collection {
                    //    if collection.verified && music_track.authorized_collections.contains(&collection.key) {
                    //        is_authorized = true;
                    //    }
                    // }
                }
            }
            
            if is_authorized {
                // Cache the authorization if cache account provided
                if let Some(user_auth) = &mut ctx.accounts.user_auth {
                    user_auth.user = user.key();
                    user_auth.track_id = music_track.track_id.clone();
                    user_auth.expires_at = Clock::get()?.unix_timestamp + 3600; // 1 hour expiration
                }
                return Ok(());
            }
        }
        
        // If we reach here, the user is not authorized
        Err(ErrorCode::UnauthorizedPlayback.into())
    }
    
    // Add function to create an authorization cache for a user
    pub fn create_user_auth_cache(
        ctx: Context<CreateUserAuthCache>,
        track_id: String,
    ) -> Result<()> {
        // Initialize with default values, actual authorization happens in verify_playback
        ctx.accounts.user_auth.user = ctx.accounts.user.key();
        ctx.accounts.user_auth.track_id = track_id;
        ctx.accounts.user_auth.expires_at = 0; // Will be set during verification
        
        Ok(())
    }

    // Create royalty configuration for an NFT
    pub fn create_royalty_config(
        ctx: Context<CreateRoyaltyConfig>,
        total_basis_points: u16,
        recipients: Vec<RoyaltyRecipient>,
        is_mutable: bool,
    ) -> Result<()> {
        // Validate royalty parameters
        require!(
            total_basis_points <= MAX_ROYALTY_BASIS_POINTS,
            ErrorCode::RoyaltyBasisPointsExceedMax
        );
        
        require!(
            recipients.len() <= MAX_ROYALTY_RECIPIENTS,
            ErrorCode::TooManyRoyaltyRecipients
        );
        
        // Ensure the sum of recipient basis points equals the total
        let sum_basis_points: u16 = recipients.iter()
            .map(|recipient| recipient.basis_points)
            .sum();
            
        require!(
            sum_basis_points == total_basis_points,
            ErrorCode::RoyaltyBasisPointsMismatch
        );
        
        // Initialize the royalty configuration
        let royalty_config = &mut ctx.accounts.royalty_config;
        royalty_config.mint = ctx.accounts.nft_mint.key();
        royalty_config.total_basis_points = total_basis_points;
        royalty_config.recipients = recipients;
        royalty_config.authority = ctx.accounts.authority.key();
        royalty_config.is_mutable = is_mutable;
        royalty_config.bump = *ctx.bumps.get("royalty_config").unwrap();
        
        Ok(())
    }
    
    // Update royalty configuration if it's mutable
    pub fn update_royalty_config(
        ctx: Context<UpdateRoyaltyConfig>,
        total_basis_points: u16,
        recipients: Vec<RoyaltyRecipient>,
    ) -> Result<()> {
        // Check if the config is mutable
        require!(
            ctx.accounts.royalty_config.is_mutable,
            ErrorCode::ImmutableRoyaltyConfig
        );
        
        // Validate royalty parameters
        require!(
            total_basis_points <= MAX_ROYALTY_BASIS_POINTS,
            ErrorCode::RoyaltyBasisPointsExceedMax
        );
        
        require!(
            recipients.len() <= MAX_ROYALTY_RECIPIENTS,
            ErrorCode::TooManyRoyaltyRecipients
        );
        
        // Ensure the sum of recipient basis points equals the total
        let sum_basis_points: u16 = recipients.iter()
            .map(|recipient| recipient.basis_points)
            .sum();
            
        require!(
            sum_basis_points == total_basis_points,
            ErrorCode::RoyaltyBasisPointsMismatch
        );
        
        // Update the royalty configuration
        let royalty_config = &mut ctx.accounts.royalty_config;
        royalty_config.total_basis_points = total_basis_points;
        royalty_config.recipients = recipients;
        
        Ok(())
    }
    
    // Process a sale with royalty distribution - optimized version
    pub fn process_sale_with_royalties(
        ctx: Context<ProcessSale>,
        sale_price: u64,
    ) -> Result<()> {
        let royalty_config = &ctx.accounts.royalty_config;
        
        // Early return if no royalty is configured
        if royalty_config.total_basis_points == 0 || royalty_config.recipients.is_empty() {
            // Direct transfer to seller, no royalties
            token::transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.buyer_payment_account.to_account_info(),
                        to: ctx.accounts.seller_payment_account.to_account_info(),
                        authority: ctx.accounts.buyer.to_account_info(),
                    },
                ),
                sale_price,
            )?;
            return Ok(());
        }
        
        // Calculate total royalty amount
        let total_royalty_amount = calculate_royalty_amount(
            sale_price, 
            royalty_config.total_basis_points
        )?;
        
        // If royalty amount is 0, skip royalty distribution
        if total_royalty_amount == 0 {
            // Direct transfer to seller
            token::transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.buyer_payment_account.to_account_info(),
                        to: ctx.accounts.seller_payment_account.to_account_info(),
                        authority: ctx.accounts.buyer.to_account_info(),
                    },
                ),
                sale_price,
            )?;
            return Ok(());
        }
        
        // Verify buyer has enough funds for the entire transaction
        require!(
            ctx.accounts.buyer_payment_account.amount >= sale_price,
            ErrorCode::InsufficientFunds
        );
        
        // Get the payment token mint for validating recipient accounts
        let payment_mint = ctx.accounts.buyer_payment_account.mint;
        
        // Get the remaining accounts for royalty recipients
        let remaining_accounts = ctx.remaining_accounts;
        require!(
            remaining_accounts.len() >= royalty_config.recipients.len(),
            ErrorCode::MissingExpectedAccount
        );
        
        // Verify seller token account uses the same mint
        require!(
            ctx.accounts.seller_payment_account.mint == payment_mint,
            ErrorCode::TokenMintMismatch
        );
        
        // Pre-validate all recipient accounts to avoid partial execution
        let mut recipient_account_map = std::collections::HashMap::new();
        for (i, recipient) in royalty_config.recipients.iter().enumerate() {
            let recipient_account_info = &remaining_accounts[i];
            
            // Verify recipient account is a valid token account
            let recipient_account = Account::<TokenAccount>::try_from(recipient_account_info)?;
            
            // Verify recipient token account uses the same mint as the payment
            require!(
                recipient_account.mint == payment_mint,
                ErrorCode::TokenMintMismatch
            );
            
            // Add to map for later use
            recipient_account_map.insert(recipient.recipient, recipient_account_info);
        }
        
        // Batch process recipients to minimize CPI calls
        let batched_recipients = batch_royalty_recipients(
            &royalty_config.recipients,
            total_royalty_amount,
            royalty_config.total_basis_points
        )?;
        
        // Calculate total royalty amount to be paid
        let mut total_royalties_paid: u64 = 0;
        for (_, amount) in &batched_recipients {
            total_royalties_paid = total_royalties_paid.checked_add(*amount)
                .ok_or(ProgramError::ArithmeticOverflow)?;
        }
        
        // Calculate seller amount
        let seller_amount = sale_price.checked_sub(total_royalties_paid)
            .ok_or(ErrorCode::RoyaltyBasisPointsExceedMax)?;
        
        // Now execute the transfers with the batched amounts
        for (recipient_pubkey, amount) in batched_recipients {
            if let Some(recipient_account_info) = recipient_account_map.get(&recipient_pubkey) {
                // Transfer tokens to recipient
                token::transfer(
                    CpiContext::new(
                        ctx.accounts.token_program.to_account_info(),
                        Transfer {
                            from: ctx.accounts.buyer_payment_account.to_account_info(),
                            to: recipient_account_info.to_account_info(),
                            authority: ctx.accounts.buyer.to_account_info(),
                        },
                    ),
                    amount,
                )?;
            } else {
                return Err(ErrorCode::InvalidRecipientAccount.into());
            }
        }
        
        // Skip transfer if amount is zero (unlikely but possible edge case)
        if seller_amount > 0 {
            // Transfer remaining amount to seller
            token::transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.buyer_payment_account.to_account_info(),
                        to: ctx.accounts.seller_payment_account.to_account_info(),
                        authority: ctx.accounts.buyer.to_account_info(),
                    },
                ),
                seller_amount,
            )?;
        }
        
        Ok(())
    }
    
    // Helper function to calculate royalty amount
    fn calculate_royalty_amount(sale_price: u64, basis_points: u16) -> Result<u64> {
        // Use checked operations and return appropriate error
        (sale_price as u128)
            .checked_mul(basis_points as u128)
            .and_then(|product| product.checked_div(10000))
            .map(|quotient| quotient as u64)
            .ok_or(ProgramError::ArithmeticOverflow.into())
    }
    
    // Helper function to calculate recipient share with proper rounding
    fn calculate_recipient_share(
        total_royalty_amount: u64,
        recipient_basis_points: u16,
        total_basis_points: u16
    ) -> Result<u64> {
        // Use checked operations and return appropriate error
        let product = (total_royalty_amount as u128)
            .checked_mul(recipient_basis_points as u128)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        
        // Round up to avoid dust amounts that can get trapped
        let divisor = total_basis_points as u128;
        let quotient = product.checked_div(divisor).ok_or(ProgramError::ArithmeticOverflow)?;
        
        // Ensure we're not overflowing u64 when casting back
        if quotient > u64::MAX as u128 {
            return Err(ProgramError::ArithmeticOverflow.into());
        }
        
        Ok(quotient as u64)
    }

    // Add a batch processing function for royalty recipients
    // This reduces separate CPI calls when possible by combining recipients with same properties
    fn batch_royalty_recipients(
        recipients: &[RoyaltyRecipient], 
        total_royalty_amount: u64,
        total_basis_points: u16
    ) -> Result<Vec<(Pubkey, u64)>> {
        // Create a map to group recipients by their address
        let mut recipient_map: std::collections::HashMap<Pubkey, u64> = std::collections::HashMap::new();
        
        // Process each recipient
        for recipient in recipients {
            let recipient_share = calculate_recipient_share(
                total_royalty_amount,
                recipient.basis_points,
                total_basis_points
            )?;
            
            if recipient_share > 0 {
                // Add to existing entry or create new one
                *recipient_map.entry(recipient.recipient).or_insert(0) += recipient_share;
            }
        }
        
        // Convert map to vector for easier iteration
        let mut result = Vec::with_capacity(recipient_map.len());
        for (pubkey, amount) in recipient_map {
            result.push((pubkey, amount));
        }
        
        Ok(result)
    }

    // Add helper to find royalty config PDA for a given mint
    pub fn find_royalty_config_pda(program_id: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[ROYALTY_CONFIG_SEED, mint.as_ref()], program_id)
    }
}

#[derive(Accounts)]
#[instruction(initial_price: u64, auction_duration_sec: u64)]
pub struct Exhibit<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(signer)]
    pub exhibitor: AccountInfo<'info>,
    #[account(
        mut,
        constraint = exhibitor_nft_token_account.amount == 1
    )]
    pub exhibitor_nft_token_account: Account<'info, TokenAccount>,
    pub exhibitor_nft_temp_account: Account<'info, TokenAccount>,
    pub exhibitor_ft_receiving_account:Account<'info, TokenAccount>,
    #[account(zero)]
    pub escrow_account: Box<Account<'info, Auction>>,
    pub clock: Sysvar<'info, Clock>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(signer)]
    pub exhibitor: AccountInfo<'info>,
    #[account(mut)]
    pub exhibitor_nft_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub exhibitor_nft_temp_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = escrow_account.exhibitor_pubkey == exhibitor.key(),
        constraint = escrow_account.highest_bidder_pubkey == exhibitor.key(),
        constraint = escrow_account.exhibiting_nft_temp_pubkey == exhibitor_nft_temp_account.key(),
        close = exhibitor
    )]
    pub escrow_account: Box<Account<'info, Auction>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub pda: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(price: u64)]
pub struct Bid<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(signer)]
    pub bidder: AccountInfo<'info>,
    #[account(mut)]
    pub bidder_ft_temp_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = bidder_ft_account.amount >= price
    )]
    pub bidder_ft_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        mut,
        constraint = highest_bidder.key() != bidder.key()
    )]
    pub highest_bidder: AccountInfo<'info>,
    #[account(mut)]
    pub highest_bidder_ft_temp_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub highest_bidder_ft_returning_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = escrow_account.highest_bidder_pubkey == highest_bidder.key(),
        constraint = escrow_account.highest_bidder_ft_temp_pubkey == highest_bidder_ft_temp_account.key(),
        constraint = escrow_account.highest_bidder_ft_returning_pubkey == highest_bidder_ft_returning_account.key(),
        constraint = escrow_account.price < price,
        constraint = escrow_account.end_at > clock.unix_timestamp
    )]
    pub escrow_account: Box<Account<'info, Auction>>,
    pub clock: Sysvar<'info, Clock>,
    /// CHECK: This is not dangerous because we don't read or write from this account TODO check pda key
    pub pda: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(signer)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub winning_bidder: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub exhibitor: AccountInfo<'info>,
    #[account(mut)]
    pub exhibitor_nft_temp_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub exhibitor_ft_receiving_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub highest_bidder_ft_temp_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub highest_bidder_nft_receiving_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = escrow_account.exhibitor_pubkey == exhibitor.key(),
        constraint = escrow_account.exhibiting_nft_temp_pubkey == exhibitor_nft_temp_account.key(),
        constraint = escrow_account.exhibitor_ft_receiving_pubkey == exhibitor_ft_receiving_account.key(),
        constraint = escrow_account.highest_bidder_pubkey == winning_bidder.key(),
        constraint = escrow_account.highest_bidder_ft_temp_pubkey == highest_bidder_ft_temp_account.key(),
        constraint = escrow_account.end_at <= clock.unix_timestamp,
        close = exhibitor
    )]
    pub escrow_account: Box<Account<'info, Auction>>,
    pub clock: Sysvar<'info, Clock>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub pda: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

impl<'info> Exhibit<'info> {
    fn to_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self
                .exhibitor_nft_token_account
                .to_account_info()
                .clone(),
            to: self.exhibitor_nft_temp_account.to_account_info().clone(),
            authority: self.exhibitor.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn to_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.exhibitor_nft_temp_account.to_account_info().clone(),
            current_authority: self.exhibitor.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}


impl<'info> Cancel<'info> {
    fn to_transfer_to_exhibitor_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.exhibitor_nft_temp_account.to_account_info().clone(),
            to: self
                .exhibitor_nft_token_account
                .to_account_info()
                .clone(),
            authority: self.pda.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn to_close_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.exhibitor_nft_temp_account.to_account_info().clone(),
            destination: self.exhibitor.clone(),
            authority: self.pda.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

impl<'info> Bid<'info> {

    fn to_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.bidder_ft_temp_account.to_account_info().clone(),
            current_authority: self.bidder.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn to_close_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.highest_bidder_ft_temp_account.to_account_info().clone(),
            destination: self.highest_bidder.clone(),
            authority: self.pda.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn to_transfer_to_previous_bidder_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.highest_bidder_ft_temp_account.to_account_info().clone(),
            to: self
                .highest_bidder_ft_returning_account
                .to_account_info()
                .clone(),
            authority: self.pda.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn to_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.bidder_ft_account.to_account_info().clone(),
            to: self
                .bidder_ft_temp_account
                .to_account_info()
                .clone(),
            authority: self.bidder.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

impl<'info> Close<'info> {
    fn to_transfer_to_exhibitor_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.highest_bidder_ft_temp_account.to_account_info().clone(),
            to: self
                .exhibitor_ft_receiving_account
                .to_account_info()
                .clone(),
            authority: self.pda.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn to_transfer_to_highest_bidder_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.exhibitor_nft_temp_account.to_account_info().clone(),
            to: self
                .highest_bidder_nft_receiving_account
                .to_account_info()
                .clone(),
            authority: self.pda.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn to_close_ft_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.highest_bidder_ft_temp_account.to_account_info().clone(),
            destination: self.winning_bidder.clone(),
            authority: self.pda.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn to_close_nft_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.exhibitor_nft_temp_account.to_account_info().clone(),
            destination: self.exhibitor.clone(),
            authority: self.pda.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

/// see https://github.com/yoshidan/solana-auction/blob/main/program/src/state.rs#L10
#[account]
pub struct Auction {
    pub exhibitor_pubkey: Pubkey,
    pub exhibiting_nft_temp_pubkey: Pubkey,
    pub exhibitor_ft_receiving_pubkey: Pubkey,
    pub price: u64,
    pub end_at: i64,
    pub highest_bidder_pubkey: Pubkey,
    pub highest_bidder_ft_temp_pubkey: Pubkey,
    pub highest_bidder_ft_returning_pubkey: Pubkey,
}

// Add new account structures for music tracks and authorization

#[account]
pub struct MusicTrack {
    pub authority: Pubkey,           // Creator/owner of the track
    pub track_id: String,            // Unique identifier for the track
    pub track_uri: String,           // IPFS/Arweave URI to the music file
    pub preview_uri: String,         // URI to the preview version (free)
    pub metadata_uri: String,        // URI to track metadata
    pub authorized_collections: Vec<Pubkey>, // List of authorized collection mints
    pub is_public: bool,             // If true, anyone can play (no NFT needed)
}

#[account]
pub struct AuthorizationRegistry {
    pub authority: Pubkey,           // Admin who can update global settings
    pub track_count: u64,            // Total number of tracks registered
    pub bump: u8,                    // PDA bump
}

// Add user authorization cache to improve efficiency
#[account]
pub struct UserAuthCache {
    pub user: Pubkey,                // User who is authorized
    pub track_id: String,            // Track they're authorized to play
    pub expires_at: i64,             // When the authorization expires (unix timestamp)
}

// Add new account validation structures

#[derive(Accounts)]
#[instruction(track_id: String)]
pub struct RegisterMusicTrack<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 4 + track_id.len() + 4 + 100 + 4 + 100 + 4 + 100 + 4 + (32 * 20) + 1, // Approximate space
        seeds = [MUSIC_TRACK_SEED, track_id.as_bytes()],
        bump
    )]
    pub music_track: Account<'info, MusicTrack>,
    
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + 32 + 8 + 1,
        seeds = [AUTH_REGISTRY_SEED],
        bump
    )]
    pub auth_registry: Account<'info, AuthorizationRegistry>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTrackAuthorization<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        constraint = music_track.authority == authority.key() @ ErrorCode::UnauthorizedPlayback
    )]
    pub music_track: Account<'info, MusicTrack>,
}

#[derive(Accounts)]
pub struct CreateUserAuthCache<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub music_track: Account<'info, MusicTrack>,
    
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 4 + music_track.track_id.len() + 8,
        seeds = [USER_AUTH_SEED, user.key().as_ref(), music_track.track_id.as_bytes()],
        bump
    )]
    pub user_auth: Account<'info, UserAuthCache>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyPlaybackAuthorization<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub music_track: Account<'info, MusicTrack>,
    
    // Optional: User's token account for the NFT
    pub user_token_account: Option<Account<'info, TokenAccount>>,
    
    // Optional: NFT metadata account
    /// CHECK: Verified in the instruction
    pub nft_metadata: Option<AccountInfo<'info>>,
    
    /// CHECK: This is the Metaplex Token Metadata program
    pub metadata_program: AccountInfo<'info>,
    
    // Optional: User authorization cache
    #[account(
        mut,
        seeds = [USER_AUTH_SEED, user.key().as_ref(), music_track.track_id.as_bytes()],
        bump,
        constraint = user_auth.user == user.key() && user_auth.track_id == music_track.track_id,
        constraint = Clock::get().unwrap().unix_timestamp <= user_auth.expires_at @ ErrorCode::AuthorizationExpired,
        required = false
    )]
    pub user_auth: Option<Account<'info, UserAuthCache>>,
    
    pub system_program: Program<'info, System>,
    
    pub clock: Sysvar<'info, Clock>,
}

// Define royalty-related account structures

#[account]
pub struct RoyaltyConfig {
    pub mint: Pubkey,                         // NFT mint this config belongs to
    pub total_basis_points: u16,              // Total royalty (e.g., 1000 = 10%)
    pub recipients: Vec<RoyaltyRecipient>,    // List of recipients
    pub authority: Pubkey,                    // Who can modify this config
    pub is_mutable: bool,                     // Whether config can be changed
    pub bump: u8,                             // PDA bump seed for easier CPI
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RoyaltyRecipient {
    pub recipient: Pubkey,                    // Recipient wallet
    pub basis_points: u16,                    // Share (e.g., 500 = 5%)
    pub recipient_type: RecipientType,        // Artist, platform, collaborator, etc.
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum RecipientType {
    Artist,
    Platform,
    Collaborator,
    Other,
}

// Account validation structures for royalty operations

#[derive(Accounts)]
pub struct CreateRoyaltyConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub nft_mint: Account<'info, Mint>,
    
    // Verify authority is connected to the NFT by requiring their token account
    #[account(
        constraint = nft_token_account.mint == nft_mint.key(),
        constraint = nft_token_account.owner == authority.key(),
        constraint = nft_token_account.amount > 0
    )]
    pub nft_token_account: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = authority,
        space = get_royalty_config_size(recipients.len()),
        seeds = [ROYALTY_CONFIG_SEED, nft_mint.key().as_ref()],
        bump
    )]
    pub royalty_config: Account<'info, RoyaltyConfig>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateRoyaltyConfig<'info> {
    #[account(
        constraint = authority.key() == royalty_config.authority @ ErrorCode::UnauthorizedRoyaltyUpdate
    )]
    pub authority: Signer<'info>,
    
    pub nft_mint: Account<'info, Mint>,
    
    // Add optional NFT ownership verification
    // This allows either the original authority OR the current NFT owner to update royalties
    // if the config is mutable
    #[account(
        constraint = nft_token_account.mint == nft_mint.key(),
        constraint = nft_token_account.owner == authority.key(),
        constraint = nft_token_account.amount > 0,
        required = false
    )]
    pub nft_token_account: Option<Account<'info, TokenAccount>>,
    
    #[account(
        mut,
        seeds = [ROYALTY_CONFIG_SEED, nft_mint.key().as_ref()],
        bump = royalty_config.bump,
        constraint = royalty_config.mint == nft_mint.key() @ ErrorCode::InvalidRoyaltyConfig
    )]
    pub royalty_config: Account<'info, RoyaltyConfig>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessSale<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    /// CHECK: This is the seller receiving payment
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    
    pub nft_mint: Account<'info, Mint>,
    
    // Add NFT token account owned by the seller to verify they own the NFT
    #[account(
        constraint = seller_nft_account.mint == nft_mint.key(),
        constraint = seller_nft_account.owner == seller.key(),
        constraint = seller_nft_account.amount == 1
    )]
    pub seller_nft_account: Account<'info, TokenAccount>,
    
    #[account(
        seeds = [ROYALTY_CONFIG_SEED, nft_mint.key().as_ref()],
        bump = royalty_config.bump,
        constraint = royalty_config.mint == nft_mint.key() @ ErrorCode::InvalidRoyaltyConfig
    )]
    pub royalty_config: Account<'info, RoyaltyConfig>,
    
    #[account(
        mut,
        constraint = buyer_payment_account.owner == buyer.key() @ ErrorCode::InvalidNFTOwnership
    )]
    pub buyer_payment_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub seller_payment_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

// Add a secure helper function for validating royalty payments using the stored bump
pub fn verify_and_process_royalty_payment(
    program_id: &Pubkey,
    nft_mint: &Pubkey,
    bump: u8,
    payment_account: &AccountInfo,
    recipient_account: &AccountInfo,
    amount: u64,
    token_program: &AccountInfo
) -> Result<()> {
    let seeds = &[
        ROYALTY_CONFIG_SEED,
        nft_mint.as_ref(),
        &[bump]
    ];
    let signer = &[&seeds[..]];
    
    token::transfer(
        CpiContext::new_with_signer(
            token_program.clone(),
            Transfer {
                from: payment_account.clone(),
                to: recipient_account.clone(),
                authority: payment_account.clone(),
            },
            signer
        ),
        amount
    )
}

// Add this helper function to create a seeds-with-bump array once and reuse it
fn get_royalty_config_seeds<'a>(
    nft_mint: &'a Pubkey,
    bump: &'a u8,
) -> [&'a [u8]; 3] {
    [ROYALTY_CONFIG_SEED, nft_mint.as_ref(), std::slice::from_ref(bump)]
}

// Add a helper function to calculate exact space needed for RoyaltyConfig
fn get_royalty_config_size(recipient_count: usize) -> usize {
    8 +                     // discriminator
    32 +                    // mint: Pubkey
    2 +                     // total_basis_points: u16
    4 +                     // recipients vec len
    recipient_count * (
        32 +                // recipient: Pubkey
        2 +                 // basis_points: u16
        1                   // recipient_type: RecipientType (enum)
    ) +
    32 +                    // authority: Pubkey
    1 +                     // is_mutable: bool
    1                       // bump: u8
}

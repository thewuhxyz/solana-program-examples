use std::ffi::CStr;

use super::SteelInstruction;
use crate::state::MintAuthorityPda;
use mpl_token_metadata::instructions as mpl_instruction;
use solana_program::{msg, program::invoke, program_pack::Pack, rent::Rent, system_instruction};
use spl_token::state::Mint;
use steel::*;

instruction!(SteelInstruction, CreateToken);
// MintTo instruction
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateToken {
    pub token_name: [u8; 32],
    pub token_symbol: [u8; 10],
    pub token_uri: [u8; 256],
}

impl CreateToken {
    pub fn process(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
        let args = Self::try_from_bytes(data)?;

        let [mint_account, mint_authority, metadata_account, payer, rent, system_program, token_program, token_metadata_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        let (mint_authority_key, bump) =
            Pubkey::find_program_address(&[MintAuthorityPda::SEED_PREFIX], &crate::ID);

        mint_authority.has_address(&mint_authority_key)?;

        // First create the account for the Mint
        //
        msg!("Creating mint account...");
        msg!("Mint: {}", mint_account.key);
        invoke(
            &system_instruction::create_account(
                payer.key,
                mint_account.key,
                (Rent::get()?).minimum_balance(Mint::LEN),
                Mint::LEN as u64,
                token_program.key,
            ),
            &[
                mint_account.clone(),
                payer.clone(),
                system_program.clone(),
                token_program.clone(),
            ],
        )?;

        // Now initialize that account as a Mint (standard Mint)
        //
        msg!("Initializing mint account...");
        msg!("Mint: {}", mint_account.key);

        initialize_mint(
            mint_account,
            mint_authority,
            Some(mint_authority),
            token_program,
            rent,
            0, // 0 Decimals for the NFT standard
        )?;

        // Now create the account for that Mint's metadata
        //
        msg!("Creating metadata account...");
        msg!("Metadata account address: {}", metadata_account.key);

        let name = CStr::from_bytes_until_nul(&args.token_name)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let symbol = CStr::from_bytes_until_nul(&args.token_symbol)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let uri = CStr::from_bytes_until_nul(&args.token_uri)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        mpl_instruction::CreateMetadataAccountV3Cpi {
            __program: token_metadata_program,
            metadata: metadata_account,
            mint: mint_account,
            mint_authority,
            payer,
            update_authority: (mint_authority, true),
            system_program,
            rent: Some(rent),
            __args: mpl_token_metadata::instructions::CreateMetadataAccountV3InstructionArgs {
                data: mpl_token_metadata::types::DataV2 {
                    name,
                    symbol,
                    uri,
                    seller_fee_basis_points: 0,
                    creators: None,
                    collection: None,
                    uses: None,
                },
                is_mutable: true,
                collection_details: None,
            },
        }
        .invoke_signed(&[&[MintAuthorityPda::SEED_PREFIX, &[bump]]])?;

        msg!("Token mint created successfully.");

        Ok(())
    }
}

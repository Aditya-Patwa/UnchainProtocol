use anchor_lang::prelude::*;
use arcium_anchor::prelude::*;
use arcium_client::idl::arcium::types::CallbackAccount;

const COMP_DEF_OFFSET_INIT_CREATORS_BALANCE: u32 = comp_def_offset("init_creators_balance");
const COMP_DEF_OFFSET_TIP_CREATOR: u32 = comp_def_offset("tip_creator");

declare_id!("FARbCiMP2XJnV44D2V79dyCTTkn4arUXXqXZGdcZsAP2");

// gMPC ☂️
#[arcium_program]
pub mod unchain_protocol {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        msg!("Initializing Vault!");

        ctx.accounts.vault.bump = ctx.bumps.vault;

        Ok(())
    }

    pub fn init_creators_balance_comp_def(ctx: Context<InitCreatorsBalanceCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, true, 0, None, None)?;
        Ok(())
    }


    pub fn become_a_creator(
        ctx: Context<BecomeACreator>,
        computation_offset: u64,
        name: String, 
        title: String, 
        about: String, 
        image_cid: String,
        unchain_profile: [u8; 32],
        initial_tip: [u8;32],
        nonce: u128,
    ) -> Result<()> {
        msg!("Becoming a Creator!");

        // Creator's Public Profile Details
        ctx.accounts.creator_profile.name = name;
        ctx.accounts.creator_profile.title = title;
        ctx.accounts.creator_profile.about = about;
        ctx.accounts.creator_profile.image_cid = image_cid;
        ctx.accounts.creator_profile.authority = ctx.accounts.payer.key();
        ctx.accounts.creator_profile.bump = ctx.bumps.creator_profile;


        // Creator's Unchain Protocol Account Details <encrypted>
        ctx.accounts.creator_account.unchain_profile = unchain_profile;
        ctx.accounts.creator_account.total_tips = [0;32];
        ctx.accounts.creator_account.unchaimed_tips = [0;32];
        ctx.accounts.creator_account.nonce = nonce;
        ctx.accounts.creator_account.bump = ctx.bumps.creator_account;
        

        let args = vec![
            Argument::ArcisPubkey(unchain_profile),
            Argument::PlaintextU128(nonce),
            Argument::EncryptedU64(initial_tip)
        ];
        
        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;     
        
        
        // Initialize <encrypted> total_tips and unchaimed_tips through MPC
        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            None,
            vec![InitCreatorsBalanceCallback::callback_ix(&[CallbackAccount {
                pubkey: ctx.accounts.creator_account.key(),
                is_writable: true,
            }])],
        )?;


        Ok(())
        
    }


    #[arcium_callback(encrypted_ix = "init_creators_balance")]
    pub fn init_creators_balance_callback(
        ctx: Context<InitCreatorsBalanceCallback>,
        output: ComputationOutputs<InitCreatorsBalanceOutput>,
    ) -> Result<()> {
        let o = match output {
            ComputationOutputs::Success(InitCreatorsBalanceOutput { field_0 }) => field_0,
            _ => return Err(ErrorCode::AbortedComputation.into()),
        };

        ctx.accounts.creator_account.total_tips = o.ciphertexts[0];
        ctx.accounts.creator_account.unchaimed_tips = o.ciphertexts[1];
        ctx.accounts.creator_account.nonce = o.nonce;

        
        emit!(InitCreatorEvent {
            total_tips: o.ciphertexts[0],
            unchaimed_tips: o.ciphertexts[1],
            nonce: o.nonce.to_le_bytes(),
        });


        Ok(())
    }



    
    pub fn init_tip_creator_comp_def(ctx: Context<InitTipCreatorCompDef>) -> Result<()> {
        init_comp_def(ctx.accounts, true, 0, None, None)?;
        Ok(())
    }

    pub fn tip_creator(
        ctx: Context<TipCreator>,
        computation_offset: u64,
        amount: [u8;32],
        pub_key: [u8; 32],
        nonce: u128,
    ) -> Result<()> {
        msg!("Sending Tip!");

        let args = vec![
            Argument::ArcisPubkey(pub_key),
            Argument::PlaintextU128(nonce),
            Argument::EncryptedU64(amount),
            Argument::PlaintextU128(ctx.accounts.creator_account.nonce),
            Argument::Account(
                ctx.accounts.creator_account.key(),
                // Offset calculation: 8 bytes (discriminator) + 1 byte (bump)
                8 + 1,
                32 * 2, // 2 encrypted tips(total_tips, unclaimed_tips), 32 bytes each
            ),
        ];

        ctx.accounts.sign_pda_account.bump = ctx.bumps.sign_pda_account;

        queue_computation(
            ctx.accounts,
            computation_offset,
            args,
            None,
            vec![TipCreatorCallback::callback_ix(&[CallbackAccount {
                pubkey: ctx.accounts.creator_account.key(),
                is_writable: true,
            }])],
        )?;
        Ok(())
    }




    
    #[arcium_callback(encrypted_ix = "tip_creator")]
    pub fn tip_creator_callback(
        ctx: Context<TipCreatorCallback>,
        output: ComputationOutputs<TipCreatorOutput>,
    ) -> Result<()> {
        let o = match output {
            ComputationOutputs::Success(TipCreatorOutput { field_0 }) => field_0,
            _ => return Err(ErrorCode::AbortedComputation.into()),
        };

        ctx.accounts.creator_account.total_tips = o.ciphertexts[0];
        ctx.accounts.creator_account.unchaimed_tips = o.ciphertexts[1];
        ctx.accounts.creator_account.nonce = o.nonce;


        emit!(TipEvent {
            total_tips: o.ciphertexts[0],
            unchaimed_tips: o.ciphertexts[1],
            nonce: o.nonce.to_le_bytes(),
        });

        Ok(())
    }
}




#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer=payer,
        space=8+1,
        seeds=[b"vault"],
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>
}


#[queue_computation_accounts("init_creators_balance", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64, name: String, title: String, about: String, image_cid: String)]
pub struct BecomeACreator<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        space = 9,
        payer = payer,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, SignerAccount>,
    #[account(
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Account<'info, MXEAccount>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_INIT_CREATORS_BALANCE)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        init,
        payer = payer, 
        space = 8 + 32 + 4 + name.len() + 4 + title.len() + 4 + about.len() + 4 + image_cid.len(),
        seeds=[b"profile", payer.key().as_ref()],
        bump,
    )]
    pub creator_profile: Account<'info, Creator>,
    #[account(
        init,
        payer=payer,
        space = 8 + CreatorAccount::INIT_SPACE,
        seeds=[b"account", payer.key().as_ref()],
        bump
    )]
    pub creator_account: Account<'info, CreatorAccount>,
}



#[callback_accounts("init_creators_balance")]
#[derive(Accounts)]
pub struct InitCreatorsBalanceCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_INIT_CREATORS_BALANCE)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: instructions_sysvar, checked by the account constraint
    pub instructions_sysvar: AccountInfo<'info>,
    #[account(mut)]
    pub creator_account: Account<'info, CreatorAccount>,
}



#[init_computation_definition_accounts("init_creators_balance", payer)]
#[derive(Accounts)]
pub struct InitCreatorsBalanceCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: comp_def_account, checked by arcium program.
    /// Can't check it here as it's not initialized yet.
    pub comp_def_account: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}







#[init_computation_definition_accounts("tip_creator", payer)]
#[derive(Accounts)]
pub struct InitTipCreatorCompDef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Box<Account<'info, MXEAccount>>,
    #[account(mut)]
    /// CHECK: comp_def_account, checked by arcium program.
    /// Can't check it here as it's not initialized yet.
    pub comp_def_account: UncheckedAccount<'info>,
    pub arcium_program: Program<'info, Arcium>,
    pub system_program: Program<'info, System>,
}


#[queue_computation_accounts("tip_creator", payer)]
#[derive(Accounts)]
#[instruction(computation_offset: u64)]
pub struct TipCreator<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        space = 9,
        payer = payer,
        seeds = [&SIGN_PDA_SEED],
        bump,
        address = derive_sign_pda!(),
    )]
    pub sign_pda_account: Account<'info, SignerAccount>,
    #[account(
        address = derive_mxe_pda!()
    )]
    pub mxe_account: Account<'info, MXEAccount>,
    #[account(
        mut,
        address = derive_mempool_pda!()
    )]
    /// CHECK: mempool_account, checked by the arcium program.
    pub mempool_account: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_execpool_pda!()
    )]
    /// CHECK: executing_pool, checked by the arcium program.
    pub executing_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        address = derive_comp_pda!(computation_offset)
    )]
    /// CHECK: computation_account, checked by the arcium program.
    pub computation_account: UncheckedAccount<'info>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_TIP_CREATOR)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(
        mut,
        address = derive_cluster_pda!(mxe_account)
    )]
    pub cluster_account: Account<'info, Cluster>,
    #[account(
        mut,
        address = ARCIUM_FEE_POOL_ACCOUNT_ADDRESS,
    )]
    pub pool_account: Account<'info, FeePool>,
    #[account(
        address = ARCIUM_CLOCK_ACCOUNT_ADDRESS
    )]
    pub clock_account: Account<'info, ClockAccount>,
    pub system_program: Program<'info, System>,
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        seeds=[b"vault"],
        bump=vault.bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub creator_account: Account<'info, CreatorAccount>
}


#[callback_accounts("tip_creator")]
#[derive(Accounts)]
pub struct TipCreatorCallback<'info> {
    pub arcium_program: Program<'info, Arcium>,
    #[account(
        address = derive_comp_def_pda!(COMP_DEF_OFFSET_TIP_CREATOR)
    )]
    pub comp_def_account: Account<'info, ComputationDefinitionAccount>,
    #[account(address = ::anchor_lang::solana_program::sysvar::instructions::ID)]
    /// CHECK: instructions_sysvar, checked by the account constraint
    pub instructions_sysvar: AccountInfo<'info>,
    #[account(mut)]
    pub creator_account: Account<'info, CreatorAccount>,
}



#[event]
pub struct TipEvent {
    // Total tips earned <encrypted>
    total_tips: [u8; 32],
    // Total unchaimed tips <encrypted>
    unchaimed_tips: [u8; 32],
    pub nonce: [u8; 16],
}

#[event]
pub struct InitCreatorEvent {
    // Total tips earned <encrypted>
    total_tips: [u8; 32],
    // Total unchaimed tips <encrypted>
    unchaimed_tips: [u8; 32],
    pub nonce: [u8; 16],
}

#[error_code]
pub enum ErrorCode {
    #[msg("The computation was aborted")]
    AbortedComputation,
    #[msg("Cluster not set")]
    ClusterNotSet,
}


/// Vault Account
#[account]
pub struct Vault {
    bump: u8
}


/// Public profile of creator
#[account]
pub struct Creator {
    authority: Pubkey,
    name: String,
    title: String,
    about: String,
    image_cid: String,
    bump: u8
}


/// Represent a confidental creator's account with <encrypted> tips
#[account]
#[derive(InitSpace)]
pub struct CreatorAccount {
    bump: u8,
    // Total tips earned <encrypted>
    total_tips: [u8; 32],
    // Total unchaimed tips <encrypted>
    unchaimed_tips: [u8; 32],
    // arcium x25519 profile pubkey
    unchain_profile: [u8;32],
    // Cryptographic nonce for the <encrypted> creator profile
    pub nonce: u128,
}
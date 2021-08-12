use anchor_lang::prelude::*;

#[program]
pub mod playground {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, store_data: [u8; 3]) -> ProgramResult {
        let store = &mut ctx.accounts.store;
        store.owner = *ctx.accounts.authority.key;
        store.data = store_data;

        Ok(())
    }

    pub fn init_global_state(
        ctx: Context<InitGlobalState>,
        admin_data: [u8; 3],
        public_data: [u8; 3],
        _bump: u8,
    ) -> ProgramResult {
        let global = &mut ctx.accounts.global;

        let store = Store {
            owner: *ctx.accounts.authority.key,
            data: public_data,
        };

        global.admin = *ctx.accounts.authority.key;
        global.public_data = store;
        global.admin_data = admin_data;
        Ok(())
    }

    pub fn modify_state(
        ctx: Context<ModifyState>,
        public_data: [u8; 3],
        _bump: u8,
    ) -> ProgramResult {
        let global_state = &mut ctx.accounts.global;
        let store = Store {
            owner: *ctx.accounts.authority.key,
            data: public_data,
        };

        global_state.public_data = store;
        Ok(())
    }

    #[access_control(ModifyPrivilegedState::check_signer_privileges(&ctx))]
    pub fn modify_privileged_state(
        ctx: Context<ModifyPrivilegedState>,
        admin_data: [u8; 3],
        _bump: u8,
    ) -> ProgramResult {
        let global_state = &mut ctx.accounts.global;
        global_state.admin_data = admin_data;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init)]
    pub store: ProgramAccount<'info, Store>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(_bump: u8)]
pub struct InitGlobalState<'info> {
    #[account(init, seeds = ["global".as_bytes(), &[_bump]], payer=authority)]
    pub global: ProgramAccount<'info, Global>,
    pub system_program: AccountInfo<'info>,

    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(_bump: u8)]
pub struct ModifyState<'info> {
    #[account(mut, seeds= ["global".as_bytes(), &[_bump]])]
    pub global: ProgramAccount<'info, Global>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(_bump: u8)]
pub struct ModifyPrivilegedState<'info> {
    #[account(mut, seeds= ["global".as_bytes(), &[_bump]])]
    pub global: ProgramAccount<'info, Global>,

    #[account(signer)]
    authority: AccountInfo<'info>,
}

impl<'info> ModifyPrivilegedState<'info> {
    fn check_signer_privileges(ctx: &Context<ModifyPrivilegedState>) -> ProgramResult {
        if ctx.accounts.global.admin != *ctx.accounts.authority.key {
            return Err(ErrorCode::InvalidAuthority.into());
        }

        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct Global {
    pub admin: Pubkey,
    pub admin_data: [u8; 3],
    pub public_data: Store,
}

#[account]
#[derive(Default)]
pub struct Store {
    pub owner: Pubkey,
    pub data: [u8; 3],
}

#[error]
pub enum ErrorCode {
    #[msg("Invalid state authority")]
    InvalidAuthority,
}

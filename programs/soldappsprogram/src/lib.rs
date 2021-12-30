use anchor_lang::prelude::*;

declare_id!("BQ4nBpT2Jxf5g4vVEdRVNYULg5JBxCkKTRcKH71Xnkgo");

#[program]
pub mod soldappsprogram {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        // get a reference to the account
        let base_account = &mut ctx.accounts.base_account;
        // initialize Total_Projects
        base_account.total_projects = 0;
        Ok(())
    }

    pub fn add_project(
        ctx: Context<AddProject>,
        project_name: String,
        project_description: String,
        project_link: String,
    ) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            project_name: project_name.to_string(),
            project_description: project_description.to_string(),
            project_link: project_link.to_string(),
            user_address: *user.to_account_info().key,
        };

        base_account.project_list.push(item);
        base_account.total_projects += 1;
        Ok(())
    }
}

// attach certain variables to the StartStuffOff context
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 8 + 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddProject<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// create a custom struct for us to work with
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub project_name: String,
    pub project_description: String,
    pub project_link: String,
    pub user_address: Pubkey,
}

// tell Solana we want to store on this account
#[account]
pub struct BaseAccount {
    pub total_projects: u64,
    pub project_list: Vec<ItemStruct>,
}

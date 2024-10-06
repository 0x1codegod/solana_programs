use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("9PL5t4oFNsWcxmgzPSuyGb35Ch7DDWXA7hkx9MLxCQSH");

#[program]
mod hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, title: String, data: String) -> Result<()> {
        require!(!title.is_empty(), CustomError::EmptyTitle);
        require!(!data.is_empty(), CustomError::EmptyData);

        ctx.accounts.new_account.title = title.clone().into_bytes(); // Store as Vec<u8>
        ctx.accounts.new_account.data = data.clone().into_bytes(); // Store as Vec<u8>

        msg!("Initialized task with title: {}", title);
        Ok(())
    }

    pub fn delete_task(ctx: Context<DeleteTask>) -> Result<()> {
        let task_title = String::from_utf8(ctx.accounts.task_account.title.clone())
            .map_err(|_| CustomError::InvalidTitleEncoding)?;

        // authority authentication
        let owner = ctx.accounts.signer.key();
        require_eq!(ctx.accounts.signer.key(), owner, CustomError::Unauthorized);
        require!(!task_title.is_empty(), CustomError::EmptyTitle);
        msg!("Proceeding to delete task with title: {}", task_title);

        // Perform any necessary clean-up here before closing
        ctx.accounts.task_account.title.clear();
        ctx.accounts.task_account.data.clear();

        msg!("Data deleted successfully.");

        Ok(())
    }

    pub fn read_task(ctx: Context<ReadTask>) -> Result<()> {
        let task_title = String::from_utf8(ctx.accounts.task_account.title.clone())
            .map_err(|_| CustomError::InvalidTitleEncoding)?;

        require!(!task_title.is_empty(), CustomError::EmptyTitle);
        msg!("Fetching task with title: {}", task_title);

        let task_data = String::from_utf8(ctx.accounts.task_account.data.clone())
            .map_err(|_| CustomError::InvalidDataEncoding)?;

        msg!(
            "Retrieved successfully title: {:?}, data: {:?}",
            task_title,
            task_data
        );

        Ok(())
    }

    pub fn update_task(ctx: Context<UpdateTask>, title: String, new_data: String) -> Result<()> {
        msg!("Updating task with title: {}", title);

        let current_title = String::from_utf8(ctx.accounts.task_account.title.clone())
            .map_err(|_| CustomError::InvalidTitleEncoding)?;

        require_eq!(current_title, title, CustomError::TitleMismatch);
        msg!("Title matches, proceeding to update data.");

        // authority authentication
        let owner = ctx.accounts.signer.key();
        require_eq!(ctx.accounts.signer.key(), owner, CustomError::Unauthorized);

        require!(!new_data.is_empty(), CustomError::EmptyData);

        ctx.accounts.task_account.data = new_data.into_bytes();
        msg!("Data updated successfully.");

        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(title: String, data: String)]
pub struct Initialize<'info> {
    #[account(
        init, 
        seeds = [title.as_bytes(), signer.key().as_ref()], 
        bump, 
        payer = signer, 
        space = TaskAccount::space(title.len(), data.len()))
    ]
    pub new_account: Account<'info, TaskAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeleteTask<'info> {
    #[account(mut, close = signer)]
    pub task_account: Account<'info, TaskAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReadTask<'info> {
    #[account(mut)]
    pub task_account: Account<'info, TaskAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String, new_data: String)]
pub struct UpdateTask<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), signer.key().as_ref()], 
        bump,
        realloc = TaskAccount::space(title.len(), new_data.len()),
        realloc::payer = signer,
        realloc::zero = false 
    )]
    pub task_account: Account<'info, TaskAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TaskAccount {
    title: Vec<u8>, // Changed to Vec<u8>
    data: Vec<u8>,  // Changed to Vec<u8>
}

impl TaskAccount {
    fn space(title_len: usize, data_len: usize) -> usize {
        // Account discriminator + length of the two vectors + lengths
        8 + (title_len + data_len) + 4 + 4 // +4 for the string lengths
    }
}

#[error_code]
pub enum CustomError {
    #[msg("The title provided does not match the existing task account.")]
    TitleMismatch,
    #[msg("The title cannot be empty.")]
    EmptyTitle,
    #[msg("The data cannot be empty.")]
    EmptyData,
    #[msg("The title encoding is invalid.")]
    InvalidTitleEncoding,
    #[msg("The data encoding is invalid.")]
    InvalidDataEncoding,
    #[msg("Onlyowner can invoke this function")]
    Unauthorized,
}

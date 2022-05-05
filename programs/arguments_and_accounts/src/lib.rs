use anchor_lang::prelude::*;

declare_id!("6HAqK9Q5gqd93YgByeFERJVqGLcUNLoHuejQF5bxqFdE");

#[program]
pub mod arguments_and_accounts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;

        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;

        Ok(())
    }
}

/* 
   * The my_account field is of type Account<'info, MyAccount> 
   * and the deserialized data structure is MyAccount. 
   
   ? The my_account field is marked with the [init] attribute.
   ? This will create a new account owned by the current program, 
   ? zero initialized.  
   
   ? When using [init], one must also provide: 
   ? 1. payer, which will fund the account creation 
   ? 2. space, which defines how large the account should be 
   ? 3. and the system_program, which is required by the 
   ?    runtime for creating the account. */

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

/* Similarly, the Update accounts struct is marked with the 
   #[account(mut)] attribute. 

   ? Marking an account as mut persists any changes made upon 
   ? exiting the program. 
   
   NOTE: you must mark an account init when using it for the 
   first time and mut for persisting changes. */

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>
}

#[account]
pub struct MyAccount {
    pub data: u64
}

/* General NOTE:
   All accounts created with Anchor are laid out as follows: 
   8-byte-discriminator || borsh serialized data. The 8-byte-discriminator 
   is created from the first 8 bytes of the Sha256 hash of the account's 
   type--using the example above, sha256("account:MyAccount")[..8]. 
   The account: is a fixed prefix. 
   
   Importantly, this allows a program to know for certain an account is 
   indeed of a given type. 
   
   ! Without it, a program would be vulnerable to account injection attacks, 
   ! where a malicious user specifies an account of an unexpected type, causing 
   ! the program to do unexpected things. 
   
   On account creation, this 8-byte discriminator doesn't exist, since the 
   account storage is zeroed. The first time an Anchor program mutates an account, 
   this discriminator is prepended to the account storage array and all subsequent 
   accesses to the account (not decorated with #[account(init)]) will check for 
   this discriminator. */

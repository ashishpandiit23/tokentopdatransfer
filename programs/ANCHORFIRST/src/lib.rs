use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;
use anchor_spl::{token::{TokenAccount, Mint, Token}, associated_token::AssociatedToken};

declare_id!("AtGABXydX8hgbBeeRE46kLDDnPTuZtTd7Yi7HjwMRNSg");

#[program]
pub mod anchor_first {
    
    use super::*;
    pub fn initializestatepda(_ctx: Context<Initialisedstatepda>,_bump:u8) -> Result<()> {

        msg!("state got Initialised");
        
            Ok(())
    }

    pub fn initialisetokenpda(ctx: Context<Initialisetokenpda>,_bump1:u8) -> Result<()> {

        msg!("token got Initialised");
        let pda = ctx.accounts.tokenpda.key();
        msg!("token pda : {}",pda);
        Ok(())
    }

    pub fn sendtokenpda(ctx: Context<SendTokenPDA>,_bump1:u8,_bump2:u8,_amount:u64) -> Result<()> {
        msg!("token process start for PDA transfer...");
        let state = &mut(ctx.accounts.statepda);
        msg!("before: {}",state.amount);
        msg!("{} bump after",state.bump);
        state.bump=_bump1;
        state.amount=_amount;
        msg!("after: {}",state.amount);
        msg!("{} bump after",state.bump);
        let bump_vector=_bump1.to_le_bytes();
        let dep=&mut ctx.accounts.deposit_token_account.key();
        let sender=&ctx.accounts.owner;
        let inner=vec![sender.key.as_ref(),dep.as_ref(),"state".as_ref(),bump_vector.as_ref()];
        let outer=vec![inner.as_slice()];
     
     
        let transfer_instruction = Transfer { 
            
            from : ctx.accounts.deposit_token_account.to_account_info(),
            to : ctx.accounts.tokenpda.to_account_info(),
            authority: sender.to_account_info()
        
        };

        let cpi_ctx = CpiContext::new_with_signer(
         ctx.accounts.token_program.to_account_info(),
         transfer_instruction,
         outer.as_slice(),
     );
      
       msg!("transfer call start");
 
        anchor_spl::token::transfer(cpi_ctx, _amount)?;
        ctx.accounts.tokenpda.reload()?;
        msg!("token pda key {}",ctx.accounts.tokenpda.key());
        msg!("token after transfer to reciever in PDA {}",ctx.accounts.tokenpda.amount);
 
        msg!("succesfully transfered");
 
         Ok(())
 
     }

   
     
     pub fn sendtokenwinner(ctx: Context<SendTokenWinner>,_bump1:u8,_bump2:u8,_amount:u64) -> Result<()>
        {
        msg!("token transfer to winner started from backend...");

       let bump_vector=_bump1.to_le_bytes();
       let dep=&mut ctx.accounts.deposit_token_account.key();
       let sender=&ctx.accounts.sender;
       let inner=vec![sender.key.as_ref(),dep.as_ref(),"state".as_ref(),bump_vector.as_ref()];
       let outer=vec![inner.as_slice()];
       let transfer_instruction = Transfer{
           from: ctx.accounts.tokenpda.to_account_info(),
           to:   ctx.accounts.wallet_to_deposit_to.to_account_info(),
           authority: ctx.accounts.statepda.to_account_info()
       };
       
       let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
        outer.as_slice(),
    );

      msg!("trasnfer call start");

       anchor_spl::token::transfer(cpi_ctx, _amount)?;


        Ok(())
    }

 
}



#[derive(Accounts)]
#[instruction(_bump : u8)]
pub struct Initialisedstatepda<'info> {
    #[account(
        init,
        payer = owner,
        seeds=[owner.key.as_ref(),deposit_token_account.key().as_ref(),"state".as_ref()],
        bump,
        space=200
    )]
    statepda: Account<'info, State>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub deposit_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info,System>,

}

#[derive(Accounts)]
#[instruction(_bump : u8)]
pub struct Initialisetokenpda<'info> {

    #[account(
        init,
        seeds = [owner.key.as_ref(),deposit_token_account.key().as_ref()],
        bump,
        payer = owner,
        token::mint = mint,
        token::authority = statepda,
     )]
     
    pub tokenpda: Account<'info, TokenAccount>,
    pub statepda: Account<'info,State>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub deposit_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info,System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info,Token>,
   
}
#[derive(Accounts)]
pub struct SendTokenPDA<'info> {

    #[account(mut)]
    pub tokenpda: Account<'info, TokenAccount>,
    pub statepda: Account<'info,State>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub deposit_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info,System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info,Token>,
   
}

#[derive(Accounts)]
pub struct SendTokenWinner<'info> {

     #[account(mut)]
     pub tokenpda: Account<'info, TokenAccount>,
     pub statepda: Account<'info,State>,
     #[account(mut)]
     pub wallet_to_deposit_to: Account<'info,TokenAccount>,
     /// CHECK not read write to this account
     pub sender : AccountInfo<'info>,
     pub deposit_token_account: Account<'info, TokenAccount>,
     #[account(mut)]
     /// CHECK not read write to this account
     pub reciever: Signer<'info>, 
     pub system_program: Program<'info,System>,
     pub rent: Sysvar<'info, Rent>,
     pub associated_token_program: Program<'info, AssociatedToken>,
     pub token_program: Program<'info,Token>,
}

#[account]
#[derive(Default)]
pub struct State {
    bump: u8,
    amount: u64,           
}

#[account]

pub struct AccountUser {
    pub owner_id: Pubkey,
    pub platform_id: Pubkey,
        }

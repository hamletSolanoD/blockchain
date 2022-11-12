use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod prove1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult{
        let article_account = &mut &mut ctx.accounts.article;
        article_account.content = ("").to_string();
        article_account.author = ("").to_string();
        Ok(())
    }

    pub fn mod_article_settings(ctx: Context<Mod_article_settings>, content: String,  author: String)-> ProgramResult{

        let article = &mut ctx.accounts.article;
        article.author = author;
        article.content= content;




        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = person_that_pays,
        space=8+32+20000
    )]
    pub article: Account<'info,Article>,
    #[account(mut)]
    pub person_that_pays: Signer<'info>,
    pub system_program: Program<'info,System>
}

#[derive(Accounts)]
pub struct Mod_article_settings<'info>{
    #[account(mut)]
    pub article: Account<'info,Article>,
}




#[account]
pub struct  Article{
    pub content:String,
    pub author:String,
}


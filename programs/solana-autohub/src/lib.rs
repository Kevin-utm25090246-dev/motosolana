use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("TuProgramIDAqui1111111111111111111111111");

#[program]
pub mod solana_autohub {
    use super::*;

    // 1. Inicializar una nueva pieza en el inventario
    pub fn list_part(ctx: Context<ListPart>, brand: String, model: String, price: u64) -> Result<()> {
        let part = &mut ctx.accounts.part_account;
        part.seller = *ctx.accounts.seller.key;
        part.brand = brand;
        part.model = model;
        part.price = price; // Precio en Lamports (1 SOL = 10^9 Lamports)
        part.is_sold = false;
        
        msg!("Pieza listada: {} para {}", part.brand, part.model);
        Ok(())
    }

    // 2. Comprar la pieza
    pub fn buy_part(ctx: Context<BuyPart>) -> Result<()> {
        let part = &mut ctx.accounts.part_account;
        
        // Verificamos que no esté vendida
        if part.is_sold {
            return Err(ErrorCode::AlreadySold.into());
        }

        // Transferencia de SOL del comprador al vendedor
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.buyer.key(),
            &part.seller,
            part.price,
        );
        
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.seller_account.to_account_info(),
            ],
        )?;

        part.is_sold = true;
        msg!("Compra exitosa de la pieza");
        Ok(())
    }
}

#[account]
pub struct SparePart {
    pub seller: Pubkey,
    pub brand: String,
    pub model: String,
    pub price: u64,
    pub is_sold: bool,
}

#[derive(Accounts)]
pub struct ListPart<'info> {
    #[account(init, payer = seller, space = 8 + 32 + 50 + 50 + 8 + 1)]
    pub part_account: Account<'info, SparePart>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyPart<'info> {
    #[account(mut)]
    pub part_account: Account<'info, SparePart>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: Este es el receptor de los fondos (vendedor)
    #[account(mut)]
    pub seller_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Esta pieza ya ha sido vendida.")]
    AlreadySold,
}
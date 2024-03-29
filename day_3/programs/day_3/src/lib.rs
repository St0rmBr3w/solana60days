// use anchor_lang::prelude::*;

// declare_id!("GYmGqbCbdunXeZ4jmiD4eeP2yDLVDcA2Yp4CWuGLrTC4");

// #[program]
// pub mod day_3 {
//     use super::*;

//     pub fn boaty_mc_boatface(ctx: Context<BoatyMcBoatface>, a: u64) -> Result<()> {
//         msg!("You said {}", a);
//         Ok(())
//     }

//     pub fn add(ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
//         let sum = a + b;
//         msg!("Sum is {}", sum);
//         Ok(())
//     }

//     pub fn sub(ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
//         let difference = a - b;
//         msg!("Difference is {}", difference);
//         Ok(())
//     }

//     pub fn mul(ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
//         let product = a * b;
//         msg!("Product is {}", product);
//         Ok(())
//     }

//     pub fn div(ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
//         let quotient = a / b;
//         msg!("Quotient is {}", quotient);
//         Ok(())
//     }

//     pub fn modulus(ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
//         let modulus = a % b;
//         msg!("Modulus is {}", modulus);
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct BoatyMcBoatface {}

use anchor_lang::prelude::*;

declare_id!("8PSAL9t1RMb7BcewhsSFrRQDq61Y7YXC5kHUxMk5b39Z");

#[program]
pub mod anchor_function_tutorial {
    use super::*;

    pub fn non_empty_account_example(ctx: Context<NonEmptyAccountExample>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct NonEmptyAccountExample<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>,
}

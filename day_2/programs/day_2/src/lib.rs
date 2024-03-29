use anchor_lang::prelude::*;

declare_id!("8xtKxL4HoJU9QmfLwdBsN89PVe6TSgvWE6Hj4CLjozFm");

/// The `day_2` program module.
/// 
/// This program includes functions for basic arithmetic operations
/// and handling an array of numbers.
#[program]
pub mod day_2 {
    use super::*;

    /// Initializes the program with given parameters.
    /// 
    /// # Arguments
    ///
    /// * `ctx` - The context in which this program is executed.
    /// * `a` - A `u64` integer.
    /// * `b` - Another `u64` integer.
    /// * `message` - A `String` message.
    /// 
    /// # Errors
    ///
    /// Will return `ProgramError::InvalidArgument` if `b` is greater than `a` (to prevent underflow).
    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        // Handle the subtraction
        let c = match a.checked_sub(b) {
            Some(value) => value,
            None => {
                // Log the error
                msg!("Subtraction underflow: b is greater than a. Defaulting to 0");
                0 // Default value in case of underflow
            }
        };
    
        // Continue with the rest of the function
        msg!("You said {:?}", message);
        msg!("You sent {} and {} and {}", a, b, c);
    
        Ok(())
    }

    /// Processes an array of numbers.
    /// 
    /// # Arguments
    ///
    /// * `ctx` - The context in which this program is executed.
    /// * `arr` - A vector of `u64` integers.
    pub fn array(ctx: Context<Initialize>,
                 arr: Vec<u64>) -> Result<()> {
        // Extract the first two elements
        let x = arr[0];
        let y = arr[1];
        // Perform the power calculation
        let result = x.pow(y as u32); // Note: `pow` takes a u32, so y is cast to u32

        msg!("Your array {:?} and your power {}", arr, result);
        Ok(())
    }

    /// Calculates the cube root of a given floating-point number.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context in which this program is executed.
    /// * `a` - A floating-point number (`f32`) for which the cube root is calculated.
    pub fn cube(ctx: Context<Initialize>, a: f32) -> Result<()> {
        let cube_root = a.cbrt();

        // Log the input and its cube root
        msg!("The cube root of {:?} is {:?}", a, cube_root);
        Ok(())
    }
}

/// Represents the accounts context required for the `Initialize` function.
#[derive(Accounts)]
pub struct Initialize {}

use anchor_lang::prelude::*;

declare_id!("6DN9Xf9s56PhiSSR4cZ3XcQSzD8JvsYDwbFUAVguNk3Z");

#[program]
pub mod day_7 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn main(ctx: Context<Initialize>) -> Result<()> {
        let a: u32 = 2;
        let b: u32 = 3;
        println!("{}", add(a, b));

        let s1 = String::from("hello");
        let s2 = String::from("world");

        // if s1 and s2 are copied, this could be a huge data transfer
        // if the strings are very long
        println!("{}", concat(s1, s2));
    }

    // implementations of add() and concat() are not shown for brevity
    // this code does not compile
}

#[derive(Accounts)]
pub struct Initialize {}

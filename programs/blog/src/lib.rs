// use anchor_lang::prelude::*;

// declare_id!("5LZXDznHg7v8h1TcKRGTkuegLbqikJBQJEdvQyEXAdHC");

// #[program]
// pub mod blog {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}

use anchor_lang::prelude::*;

declare_id!("5LZXDznHg7v8h1TcKRGTkuegLbqikJBQJEdvQyEXAdHC");

#[program]
pub mod blog {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

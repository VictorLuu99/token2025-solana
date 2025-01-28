use anchor_lang::prelude::*;

declare_id!("GrnfcFpWMj1UQLNMNi4cgKxKR37yAki9H4Y5a3RqjnAG");


pub mod constants;
pub mod instructions;

use instructions::*;

#[program]
pub mod token2025 {

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        instructions::initialize::initialize(ctx, name, symbol, uri)
    }
}

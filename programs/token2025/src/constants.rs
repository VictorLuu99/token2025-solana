use anchor_lang::prelude::Pubkey;

pub const DISCRIMINATOR_SIZE: usize = std::mem::size_of::<u64>();
pub const PUBKEY_SIZE: usize = std::mem::size_of::<Pubkey>();
pub const U8_SIZE: usize = std::mem::size_of::<u8>();
pub const U32_SIZE: usize = std::mem::size_of::<u32>();
pub const U64_SIZE: usize = std::mem::size_of::<u64>();
pub const F64_SIZE: usize = std::mem::size_of::<f64>();
pub const BOOL_SIZE: usize = std::mem::size_of::<bool>();



pub const PERCENTAGE_DENOMINATOR: f64 = 10000.0;
pub const TOKEN_DECIMALS: u32 = 6;

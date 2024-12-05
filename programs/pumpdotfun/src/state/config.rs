use anchor_lang::prelude::*;


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, Debug, PartialEq)]
#[repr(u8)]
pub enum StatusFlag {
    Running,
    Migrated,
    Paused,
}


#[account(zero_copy)]
#[derive(Default, InitSpace, Debug)]
pub struct Config {
    pub authority: Pubkey,
    pub mode: u8,
    pub mint_decimals: u8,
    pub padding_0: [u8; 4],
    pub token_supply: u64,
    pub fee_bps: u64,
    pub padding_1: u64,
}
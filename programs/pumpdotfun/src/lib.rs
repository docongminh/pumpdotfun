use anchor_lang::prelude::*;


pub mod instructions;

pub mod curve;

pub mod state;

pub mod events;

declare_id!("14BsjsvfuEUZMckkodZduNyBEzR3wqRNpR3LxYy96z4h");


pub mod authority {
    use anchor_lang::prelude::declare_id;
    #[cfg(feature = "devnet")]
    declare_id!("GsuXvQaMBFRiwVAhj69ydRLDorM3DH6ekpxDe9Rz9r7Q");
    #[cfg(not(feature = "devnet"))]
    declare_id!("GsuXvQaMBFRiwVAhj69ydRLDorM3DH6ekpxDe9Rz9r7Q");
}

#[program]
pub mod pumpdotfun {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

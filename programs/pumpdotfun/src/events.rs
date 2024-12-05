use anchor_lang::prelude::*;

#[event]
pub struct CreateNewCoinEvent {}


pub trait EmitEvent<T: anchor_lang::Event> {
    fn emit_event(&self) -> T;
}
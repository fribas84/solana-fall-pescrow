use pinocchio::{
    AccountView, Address, ProgramResult, cpi::{Seed, Signer}, error::ProgramError, sysvars::{Sysvar, rent::Rent}
};
use pinocchio_system::instructions::CreateAccount;

use crate::state::Escrow;

/// 8 (amount_to_receive) + 8 (amount_to_give)
const MAKE_DATA_LEN: usize = 16;

pub fn process_make_instruction(
    accounts: &mut [AccountView],
    data: &[u8],
) -> ProgramResult {
}
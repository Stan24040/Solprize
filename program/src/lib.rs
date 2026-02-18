use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct LotteryState {
    pub jackpot: u64,
    pub total_tickets: u64,
    pub ticket_price: u64,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Solprize Lottery Program");
    msg!("Program ID: {}", program_id);
    msg!("Accounts: {}", accounts.len());
    Ok(())
}

use anchor_lang::prelude::*;

declare_id!("MDIoRAC1exxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");

#[program]
pub mod mdi_oracle {
    use super::*;

    /// Submit an oracle answer from the MDI collective
    pub fn submit_answer(
        ctx: Context<SubmitAnswer>,
        question_id: u64,
        question_hash: [u8; 32],
        answer_hash: [u8; 32],
        confidence: u8,
        agent_count: u8,
        fragment_count: u16,
    ) -> Result<()> {
        let oracle_answer = &mut ctx.accounts.oracle_answer;
        
        oracle_answer.question_id = question_id;
        oracle_answer.question_hash = question_hash;
        oracle_answer.answer_hash = answer_hash;
        oracle_answer.confidence = confidence;
        oracle_answer.agent_count = agent_count;
        oracle_answer.fragment_count = fragment_count;
        oracle_answer.timestamp = Clock::get()?.unix_timestamp;
        oracle_answer.authority = ctx.accounts.authority.key();
        oracle_answer.bump = ctx.bumps.oracle_answer;
        
        emit!(AnswerSubmitted {
            question_id,
            confidence,
            agent_count,
            timestamp: oracle_answer.timestamp,
        });
        
        Ok(())
    }

    /// Verify an oracle answer exists on-chain
    pub fn verify_answer(ctx: Context<VerifyAnswer>) -> Result<OracleAnswerData> {
        let answer = &ctx.accounts.oracle_answer;
        Ok(OracleAnswerData {
            question_id: answer.question_id,
            confidence: answer.confidence,
            agent_count: answer.agent_count,
            fragment_count: answer.fragment_count,
            timestamp: answer.timestamp,
        })
    }
}

#[derive(Accounts)]
#[instruction(question_id: u64)]
pub struct SubmitAnswer<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + OracleAnswer::INIT_SPACE,
        seeds = [b"oracle", question_id.to_le_bytes().as_ref()],
        bump
    )]
    pub oracle_answer: Account<'info, OracleAnswer>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyAnswer<'info> {
    pub oracle_answer: Account<'info, OracleAnswer>,
}

#[account]
#[derive(InitSpace)]
pub struct OracleAnswer {
    pub question_id: u64,
    pub question_hash: [u8; 32],
    pub answer_hash: [u8; 32],
    pub confidence: u8,
    pub agent_count: u8,
    pub fragment_count: u16,
    pub timestamp: i64,
    pub authority: Pubkey,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct OracleAnswerData {
    pub question_id: u64,
    pub confidence: u8,
    pub agent_count: u8,
    pub fragment_count: u16,
    pub timestamp: i64,
}

#[event]
pub struct AnswerSubmitted {
    pub question_id: u64,
    pub confidence: u8,
    pub agent_count: u8,
    pub timestamp: i64,
}

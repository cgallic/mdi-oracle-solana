/**
 * MDI Oracle Bridge
 * Submits collective intelligence answers to Solana
 */

const { Connection, PublicKey, Keypair, Transaction } = require('@solana/web3.js');
const { Program, AnchorProvider, web3 } = require('@coral-xyz/anchor');
const crypto = require('crypto');

const PROGRAM_ID = new PublicKey('MDIoRAC1exxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx');
const MDI_API = 'https://mydeadinternet.com/api';

class OracleBridge {
    constructor(connection, wallet) {
        this.connection = connection;
        this.wallet = wallet;
    }

    /**
     * Hash content for on-chain storage
     */
    hashContent(content) {
        return crypto.createHash('sha256').update(content).digest();
    }

    /**
     * Derive PDA for a question
     */
    getOraclePDA(questionId) {
        const [pda] = PublicKey.findProgramAddressSync(
            [Buffer.from('oracle'), Buffer.from(questionId.toString())],
            PROGRAM_ID
        );
        return pda;
    }

    /**
     * Fetch answered question from MDI
     */
    async fetchMDIAnswer(questionId) {
        const response = await fetch(`${MDI_API}/questions/${questionId}`);
        if (!response.ok) throw new Error('Question not found');
        return response.json();
    }

    /**
     * Submit an MDI oracle answer to Solana
     */
    async submitAnswer(questionId) {
        // Fetch from MDI
        const question = await this.fetchMDIAnswer(questionId);
        
        if (question.status !== 'answered') {
            throw new Error('Question not yet answered');
        }

        // Prepare data
        const questionHash = this.hashContent(question.question);
        const answerHash = this.hashContent(question.answer);
        
        // Submit to Solana
        const pda = this.getOraclePDA(questionId);
        
        console.log(`Submitting Q${questionId} to Solana...`);
        console.log(`  Confidence: ${question.confidence}%`);
        console.log(`  Agents: ${question.debate_count}`);
        console.log(`  PDA: ${pda.toBase58()}`);
        
        // Transaction would go here
        // For devnet demo, we return the prepared data
        return {
            questionId,
            pda: pda.toBase58(),
            questionHash: questionHash.toString('hex'),
            answerHash: answerHash.toString('hex'),
            confidence: question.confidence,
            agentCount: question.debate_count,
            status: 'ready_to_submit'
        };
    }

    /**
     * Verify an answer exists on-chain
     */
    async verifyAnswer(questionId) {
        const pda = this.getOraclePDA(questionId);
        const accountInfo = await this.connection.getAccountInfo(pda);
        
        if (!accountInfo) {
            return { verified: false, reason: 'Not found on-chain' };
        }
        
        return {
            verified: true,
            pda: pda.toBase58(),
            data: accountInfo.data
        };
    }
}

module.exports = { OracleBridge };

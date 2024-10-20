import { describe, test } from "node:test";
import { PublicKey, Transaction, TransactionInstruction } from "@solana/web3.js";
import { assert } from "chai";
import { start } from "solana-bankrun";

describe("testing hello solana logs", async () => {
    // you gotta load program in solana_bankrun doing that rn
    const PROGRAM_ID = PublicKey.unique();
    const context = await start([{ name: 'native', programId: PROGRAM_ID}], []);
    const client  = context.banksClient;
    const payer = context.payer;
    console.log("I'm inside:)");

    test('hello solana', async () => {
        console.log("I'm inside the test")
        const bloackhash = context.lastBlockhash; 

        //setting up the instruction
        const ix = new TransactionInstruction({
            keys: [{ pubkey: payer.publicKey, isSigner: true, isWritable: true}],
            programId: PROGRAM_ID,
            data: Buffer.alloc(0) //empty buffer
        });

        const tx = new Transaction();
        tx.recentBlockhash = bloackhash;
        tx.add(ix).sign(payer);

        const transaction = await client.processTransaction(tx);

        assert(transaction.logMessages[0].startsWith(`Program ${PROGRAM_ID}`));
        assert(transaction.logMessages[1]==="Program log: Hello, Solana!")
    })
})
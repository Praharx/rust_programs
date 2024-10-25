import { describe, test } from 'mocha';
import { assert } from 'chai';
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { start } from 'solana-bankrun';
// import { COUNTER_ACCOUNT_SIZE, PROGRAM_ID, createIncrementInstruction, deserializeCounterAccount } from '../ts';

describe('Counter Solana Native', () => {
  const programId = PublicKey.unique();
  // Randomly generate the program keypair and load the program to solana-bankrun
  
  test('check the logs', async () => {
    const context = await start([{ name: 'solana_bankrun', programId }], []);
    const client = context.banksClient;
    // Get the payer keypair from the context, this will be used to sign transactions with enough lamports
    const payer = context.payer;
    const blockhash = context.lastBlockhash;
    // Get the rent object to calculate rent for the accounts
    const rent = await client.getRent();
    const ixs = [
        new TransactionInstruction({
            programId,
            keys:[
                {pubkey: PublicKey.unique(), isSigner: false, isWritable: false}
            ]
        })
    ];
    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(...ixs);
    tx.sign(payer);
    
    //simulation
    const simre = await client.simulateTransaction(tx);
    const meta = await client.processTransaction(tx);
    console.log(meta?.logMessages);
    expect(true);
    });
  })

//   test('Test allocate counter + increment tx', async () => {
//     // Randomly generate the account key
//     // to sign for setting up the Counter state
//     const counterKeypair = Keypair.generate();
//     const counter = counterKeypair.publicKey;

    // Create a TransactionInstruction to interact with our counter program
//     const allocIx: TransactionInstruction = SystemProgram.createAccount({
//       fromPubkey: payer.publicKey,
//       newAccountPubkey: counter,
//       lamports: Number(rent.minimumBalance(BigInt(COUNTER_ACCOUNT_SIZE))),
//       space: COUNTER_ACCOUNT_SIZE,
//       programId: PROGRAM_ID,
//     });
//     const incrementIx: TransactionInstruction = createIncrementInstruction({ counter }, {});
//     const tx = new Transaction().add(allocIx).add(incrementIx);

//     // Explicitly set the feePayer to be our wallet (this is set to first signer by default)
//     tx.feePayer = payer.publicKey;

//     // Fetch a "timestamp" so validators know this is a recent transaction
//     const blockhash = context.lastBlockhash;
//     tx.recentBlockhash = blockhash;

//     // Sign the transaction with the payer's keypair
//     tx.sign(payer, counterKeypair);

//     // Send transaction to bankrun
//     await client.processTransaction(tx);

//     // Get the counter account info from network
//     const counterAccountInfo = await client.getAccount(counter);
//     assert(counterAccountInfo, 'Expected counter account to have been created');

//     // Deserialize the counter & check count has been incremented
//     const counterAccount = deserializeCounterAccount(Buffer.from(counterAccountInfo.data));
//     assert(counterAccount.count.toNumber() === 1, 'Expected count to have been 1');
//     console.log(`[alloc+increment] count is: ${counterAccount.count.toNumber()}`);
//   });

//   test('Test allocate tx and increment tx', async () => {
//     const counterKeypair = Keypair.generate();
//     const counter = counterKeypair.publicKey;

//     // Check allocate tx
//     const allocIx: TransactionInstruction = SystemProgram.createAccount({
//       fromPubkey: payer.publicKey,
//       newAccountPubkey: counter,
//       lamports: Number(rent.minimumBalance(BigInt(COUNTER_ACCOUNT_SIZE))),
//       space: COUNTER_ACCOUNT_SIZE,
//       programId: PROGRAM_ID,
//     });
//     let tx = new Transaction().add(allocIx);
//     const blockhash = context.lastBlockhash;
//     tx.feePayer = payer.publicKey;
//     tx.recentBlockhash = blockhash;
//     tx.sign(payer, counterKeypair);

//     await client.processTransaction(tx);

//     let counterAccountInfo = await client.getAccount(counter);
//     assert(counterAccountInfo, 'Expected counter account to have been created');

//     let counterAccount = deserializeCounterAccount(Buffer.from(counterAccountInfo.data));
//     assert(counterAccount.count.toNumber() === 0, 'Expected count to have been 0');
//     console.log(`[allocate] count is: ${counterAccount.count.toNumber()}`);

//     // Check increment tx
//     const incrementIx: TransactionInstruction = createIncrementInstruction({ counter }, {});
//     tx = new Transaction().add(incrementIx);
//     tx.feePayer = payer.publicKey;
//     tx.recentBlockhash = blockhash;
//     tx.sign(payer);

//     await client.processTransaction(tx);

//     counterAccountInfo = await client.getAccount(counter);
//     assert(counterAccountInfo, 'Expected counter account to have been created');

//     counterAccount = deserializeCounterAccount(Buffer.from(counterAccountInfo.data));
//     assert(counterAccount.count.toNumber() === 1, 'Expected count to have been 1');
//     console.log(`[increment] count is: ${counterAccount.count.toNumber()}`);
//   });
// });

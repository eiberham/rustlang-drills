const solanaWeb3 = require('@solana/web3.js');
const { SystemProgram } = solanaWeb3;
const { serialize, deserialize } = require('borsh');
const fs = require('fs');
const bufferLayout = require('buffer-layout');

async function main() {
  
  try {
    const schema = { struct: { id: 'u64', value: 'u64' } }

    // load the payer's keypair from the keypair file
    const payerKeyPair = JSON.parse(fs.readFileSync('/Users/eiberham/.config/solana/id.json', 'utf-8'));
    const payer = solanaWeb3.Keypair.fromSecretKey(new Uint8Array(payerKeyPair));

    // create a connection to the local cluster
    const connection = new solanaWeb3.Connection('http://localhost:8899');

    // create a new transaction
    const transaction = new solanaWeb3.Transaction({ feePayer: payer.publicKey });

    const data = { id: 1, value: 1 };

    const pubKey = 'CTWHuAURrUr95yVYsLF3yJpXMBDAAyGy6dDjx8fUR9Ld'
    const programId = '5s1oWsg8dd9ZV26msJhaZWLMUyoSJcFJUJQWZaBourwn'

    const space = 1024

    const newAccount = solanaWeb3.Keypair.generate();

    const createAccountInstruction = SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: newAccount.publicKey,
        lamports: await connection.getMinimumBalanceForRentExemption(space),
        space,
        programId: new solanaWeb3.PublicKey(programId),
    });

    transaction.add(createAccountInstruction);

    // add an instruction to the transaction
    transaction.add(new solanaWeb3.TransactionInstruction({
        keys: [{
            pubkey: newAccount.publicKey,
            isSigner: false,
            isWritable: true
        }], // the accounts involved in this transaction
        programId: new solanaWeb3.PublicKey(programId), // replace 'ProgramId' with the ID of the program you want to send the transaction to 
        data: Buffer.from(serialize(schema, data)) // the data you want to send to the program
    }));
  
    const recentBlockhash = await connection.getLatestBlockhash();
    transaction.recentBlockhash = recentBlockhash.blockhash;
    transaction.lastValidBlockHeight = recentBlockhash.lastValidBlockHeight

    transaction.sign(payer, newAccount);

    // serialize the transaction
    const serializedTransaction = transaction.serialize();

    // send the transaction
    const transactionId = await connection.sendRawTransaction(serializedTransaction, {
            maxRetries: 5,
            preflightCommitment: 'finalized'
        });

    console.log('Transaction ID: ', transactionId);

    // confirm the transaction
    await connection.confirmTransaction(transactionId);

    // Fetch the account data
    const accountInfo = await connection.getAccountInfo(newAccount.publicKey);
    const accountData = deserialize(schema, accountInfo.data);

    // log the account data
    console.log('Account data:', accountData);
  } catch (error) {
    console.error('Failed:', error);
  }
  
}

main().catch(console.error);
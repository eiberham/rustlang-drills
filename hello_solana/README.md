solana send --url localhost <YOUR_PUBLIC_KEY> <AMOUNT> --fee-payer <YOUR_PRIVATE_KEY> --from <YOUR_PUBLIC_KEY>

solana invoke --url localhost D8FFsYH79NBASYYcS2KmSS8PN8xADdfXCkW7w1335wQ4 --fee-payer /Users/eiberham/.config/solana/id.json

/**
 * Steps
 * cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
 * 
 * cargo init hello_solana --lib
 * avm install latest
 * solana config set --url localhost
 * solana-keygen new
 * Wrote new keypair to /Users/eiberham/.config/solana/id.json
========================================================================
pubkey: AszBUQqKBYeRFMAhKdqeNGpUJJzQLZ39cAmr4npcasRP
========================================================================
Save this seed phrase and your BIP39 passphrase to recover your new keypair:
prepare sustain output wear bulb relief exit cattle flip beef basic food
========================================================================
 *
 * solana config set -k ~/.config/solana/id.json
 * 
 * Config File: /Users/eiberham/.config/solana/cli/config.yml
    RPC URL: http://localhost:8899 
    WebSocket URL: ws://localhost:8900/ (computed)
    Keypair Path: /Users/eiberham/.config/solana/id.json 
    Commitment: confirmed 
 *
 * cargo build-bpf
 * solana program deploy ./target/deploy/hello_solana.so
 * Program Id: D8FFsYH79NBASYYcS2KmSS8PN8xADdfXCkW7w1335wQ4
 * In order to see the real time logs, run in another terminal tab:
 * solana logs


 solana-install init 1.18.1 

 eiberham@macbook hello_solana % solana-keygen recover --force
[recover] seed phrase: 
[recover] If this seed phrase has an associated passphrase, enter it now. Otherwise, press ENTER to continue: 
Recovered pubkey `"2gMWLDNHBS8qLkvZyRnASSQCuGUPDN7tDE5TPrMEpBFn"`. Continue? (y/n): 
y

Had to remove target folder and run cargo clean
then rebuild with cargo build-bpf

solana airdrop 1 2gMWLDNHBS8qLkvZyRnASSQCuGUPDN7tDE5TPrMEpBFn --url http://localhost:8899

solana-keygen new

Wrote new keypair to /Users/eiberham/.config/solana/id.json
==================================================================================
pubkey: CTWHuAURrUr95yVYsLF3yJpXMBDAAyGy6dDjx8fUR9Ld

solana airdrop 1 CTWHuAURrUr95yVYsLF3yJpXMBDAAyGy6dDjx8fUR9Ld --url http://localhost:8899
solana program deploy ./target/deploy/hello_solana.so

Program Id: 5s1oWsg8dd9ZV26msJhaZWLMUyoSJcFJUJQWZaBourwn

 */
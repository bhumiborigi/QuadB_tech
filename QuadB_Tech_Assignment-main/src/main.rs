// src/main.rs
mod token_wallet;
use token_wallet::Wallet;

fn main() {
    let bhumi_wallet = Wallet::new();
    let bob_wallet = Wallet::new();

    bhumi_wallet.receive_tokens(100);  // Add initial balance to Alice's wallet
    match bhumi_wallet.send_tokens(50, &bob_wallet) {
        Ok(_) => println!("Tokens sent successfully!"),
        Err(e) => println!("Failed to send tokens: {}", e),
    }

    println!("Bhumi's balance: {}", bhumi_wallet.get_balance());
    println!("Bob's balance: {}", bob_wallet.get_balance());
}

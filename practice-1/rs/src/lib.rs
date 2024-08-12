#[cfg(test)]
mod prereqs_tests {
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{
        signature::{Keypair, Signer, read_keypair_file},
    };
    use bs58;

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        // Create a new keypair
        let keypair = Keypair::new();
        println!("You've generated a new Solana wallet: {}", keypair.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", keypair.to_bytes());
    }

    #[test]
    fn load_keypair_and_check_balance_and_airdrop() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL);

        // Get balance of dev wallet
        let balance = client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        println!("Balance: {}", balance);

        // We're going to claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            },
            Err(e) => println!("Oops, something went wrong: {}", e.to_string())
        };
    }

    #[test]
    fn grind() {
        // еще можно использовать так :
        // solana-keygen grind --starts-and-ends-with Vadym::1 --use-mnemonic
        let prefix = "Vadym";
        println!("Поиск ключа Solana с адресом, начинающимся с '{}'...", prefix);

        loop {
            let keypair = Keypair::new();
            let public_key = keypair.pubkey();
            let address = public_key.to_string();

            if address.starts_with(prefix) {
                println!("Найден подходящий ключ!");
                println!("Публичный ключ (адрес): {}", address);
                println!("Приватный ключ (base58): {}", bs58::encode(keypair.to_bytes()).into_string());
                break;
            }
        }
    }
}

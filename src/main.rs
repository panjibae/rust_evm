use std::str::FromStr;
use rand::Rng; // Untuk menghasilkan angka acak
use web3::{
    transports::Http,
    types::{Address, TransactionParameters, U256},
    signing::SecretKey,
    Web3,
};

#[tokio::main]
async fn main() -> Result<(), web3::Error> {
    // URL node Ethereum (misalnya Infura, Alchemy)
    let transport = Http::new("https://ethereum-sepolia-rpc.publicnode.com")?;
    let web3 = Web3::new(transport);

    // Alamat pengirim ETH & private key
    let from_address: Address = "YOUR ADDRESS"
        .parse()
        .map_err(|e| web3::Error::Decoder(format!("Invalid sender address: {}", e)))?;

    let private_key = SecretKey::from_str("YOUR PRIVATE KEY")
        .map_err(|e| web3::Error::Decoder(format!("Invalid private key: {}", e)))?;

    // Jumlah ETH yang akan dikirim (dalam Wei)
    let amount = U256::from(100_000_000_000_000u64); // 0.0001 ETH in Wei

    // Estimasi gas
    let gas_price = web3.eth().gas_price().await?;
    let gas_limit = U256::from(21000); // Estimasi gas untuk transfer ETH

    // Mendapatkan nonce akun untuk pertama kalinya
    let mut nonce = web3.eth().transaction_count(from_address, None).await?;

    // Loop untuk mengirim transaksi 100 kali ke alamat acak
    for i in 0..100 {
        // Menghasilkan alamat acak
        let to_address: Address = generate_random_address();

        // Membuat transaksi
        let tx_object = TransactionParameters {
            to: Some(to_address),
            value: amount,
            gas: gas_limit,
            gas_price: Some(gas_price),
            nonce: Some(nonce),
            ..Default::default()
        };

        // Menandatangani transaksi
        let signed_tx = web3.accounts().sign_transaction(tx_object, &private_key).await?;

        // Mengirim transaksi
        let result = web3.eth().send_raw_transaction(signed_tx.raw_transaction).await?;

        println!("Transaksi {} berhasil dikirim ke {}. Hash: {:?}", i + 1, to_address, result);

        // Increment nonce untuk transaksi berikutnya
        nonce += U256::from(1); // Menambahkan 1 ke nonce untuk transaksi selanjutnya
    }

    Ok(())
}

// Fungsi untuk menghasilkan alamat Ethereum acak
fn generate_random_address() -> Address {
    let mut rng = rand::thread_rng();
    
    // Menghasilkan 20 byte acak untuk alamat Ethereum
    let random_bytes: [u8; 20] = rng.gen();
    
    Address::from(random_bytes)
}

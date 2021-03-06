use super::BlockchainNetwork;
use anyhow::{bail, Result};
use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tiny_keccak::Hasher;
use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
};
use web3::{futures::sink::Buffer, types::Address};
use crate::encryption::Encrypter;

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    id: String,
    address: BlockchainAddress,
    private_key: PrivateKey,
    network: BlockchainNetwork,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockchainAddress {
    address: String,
    pub_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrivateKey {
    private_key: [u8; 32],
}

impl Wallet {
    pub fn new(network: BlockchainNetwork) -> Self {
        let id = uuid::Uuid::new_v4();
        let (pk, pub_k) = Wallet::generate_keypair();
        let address = Wallet::public_key_address(&pub_k);

        Wallet {
            id: id.to_string(),
            address: BlockchainAddress {
                address: format!("{:?}", address),
                pub_key: pub_k.to_string(),
            },
            private_key: PrivateKey {
                private_key: pk.secret_bytes(),
            },
            network: network,
        }
    }

    pub async fn save_to_file(&self, file_path: &str, secret: String) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .await?;

        let mut serialized = serde_json::to_vec(self)?;
        //println!("{:?}", serialized);
        let encrypted = Encrypter::Encrypt(&mut serialized, secret);
        file.write(&encrypted).await?;

        Ok(())
    }

    pub async fn from_file(file_path: &str, secret: String) -> Result<Wallet> {
        let mut file = OpenOptions::new().read(true).open(file_path).await?;

        //TODO: BufferRead
        let mut buffer = vec![];

        let count = file.read_to_end(&mut buffer).await?;
        println!("{:?}", buffer);
        let decrypted = Encrypter::Decrypt(&buffer[0..count], secret)?;
        let wallet: Wallet = serde_json::from_slice(&decrypted)?;
        Ok(wallet)
    }

    fn generate_keypair() -> (SecretKey, PublicKey) {
        let secp = secp256k1::Secp256k1::new();
        let mut rng = rngs::JitterRng::new_with_timer(Wallet::get_nstime);
        secp.generate_keypair(&mut rng)
    }

    fn public_key_address(public_key: &PublicKey) -> Address {
        let public_key = public_key.serialize_uncompressed();
        debug_assert_eq!(public_key[0], 0x04);
        let mut hasher = tiny_keccak::Keccak::v256();
        let mut output = [0u8; 32];
        hasher.update(&public_key[1..]);
        hasher.finalize(&mut output);
        Address::from_slice(&output[12..])
    }

    pub fn get_nstime() -> u64 {
        let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        // The correct way to calculate the current time is
        // `dur.as_secs() * 1_000_000_000 + dur.subsec_nanos() as u64`
        // But this is faster, and the difference in terms of entropy is
        // negligible (log2(10^9) == 29.9).
        dur.as_secs() << 30 | dur.subsec_nanos() as u64
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Wallet;

    #[test]
    fn wallet_is_generated() {
        let wallet = Wallet::new(crate::domain::BlockchainNetwork::EthereumMainnet);
        println!("{:?}", wallet);
    }

    #[tokio::test]
    async fn save_load() {
        let wallet = Wallet::new(crate::domain::BlockchainNetwork::EthereumMainnet);
        let exe = std::env::current_dir().unwrap();

        let path = format!(
            "{}{}{}",
            exe.to_str().unwrap(),
            std::path::MAIN_SEPARATOR,
            "test.txt"
        );
        println!("{}", path);

        let secret = "Secret".to_owned();
        wallet.save_to_file(&path, secret.clone()).await.unwrap();
        let wallet = Wallet::from_file(&path, secret.clone()).await.unwrap();
        println!("{:?}", wallet);
    }
}

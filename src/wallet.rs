// src/wallet.rs
use bip39::{Mnemonic, Language};
use rand::rngs::OsRng;
use ed25519_dalek::{Keypair, Signer};

#[derive(Clone)]
pub struct Wallet {
    pub address: String,
    pub mnemonic: String,
    keypair: Keypair,
}

impl Wallet {
    pub fn new() -> Self {
        let mnemonic = Mnemonic::new(OsRng, Language::Vietnamese);
        let phrase = mnemonic.phrase().to_string();
        let seed = mnemonic.to_seed("");
        let keypair = Keypair::from_bytes(&seed[..32]).unwrap();
        let address = format!("PAPPAP_{:x}", &keypair.public.to_bytes()[..8]);
        
        Self { address, mnemonic: phrase, keypair }
    }

    pub fn sign(&self, msg: &[u8]) -> Vec<u8> {
        self.keypair.sign(msg).to_bytes().to_vec()
    }
}
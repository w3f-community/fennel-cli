use fennel_lib::{encrypt, import_public_key_from_binary, verify};
use jsonrpsee::core::{async_trait, Error};

use super::traits::FennelRPCServer;
use super::types::{FennelFingerprint, FennelPublicKeyBytes, FennelSignature};
use crate::client::{handle_decrypt, handle_generate_keypair, handle_sign};

pub struct FennelRPCService;

#[async_trait]
impl FennelRPCServer<FennelFingerprint, FennelSignature, FennelPublicKeyBytes>
    for FennelRPCService
{
    async fn encrypt(
        &self,
        plaintext: Vec<u8>,
        public_key_bytes: FennelPublicKeyBytes,
    ) -> Result<Vec<u8>, Error> {
        let public_key =
            import_public_key_from_binary(&public_key_bytes.try_into().unwrap()).unwrap();
        Ok(encrypt(public_key, plaintext))
    }

    async fn decrypt(&self, ciphertext: Vec<u8>) -> Result<Vec<u8>, Error> {
        let (_, private_key, _) = handle_generate_keypair();
        Ok(handle_decrypt(ciphertext, &private_key).as_bytes().to_vec())
    }

    async fn sign(&self, ciphertext: Vec<u8>) -> Result<Vec<u8>, Error> {
        let (_, private_key, _) = handle_generate_keypair();
        Ok(
            handle_sign(&String::from_utf8_lossy(&ciphertext), private_key)
                .as_bytes()
                .to_vec(),
        )
    }

    async fn verify(
        &self,
        message: Vec<u8>,
        signature: Vec<u8>,
        public_key_bytes: FennelPublicKeyBytes,
    ) -> Result<bool, Error> {
        let public_key =
            import_public_key_from_binary(&public_key_bytes.try_into().unwrap()).unwrap();
        Ok(verify(public_key, message, signature))
    }
}
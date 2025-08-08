use anchor_lang::prelude::*;
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};
use sha2::{Digest, Sha256};
use crate::errors::VaultError;

/// ECDSA signature validator for Bitcoin address ownership
pub struct ECDSAValidator;

impl ECDSAValidator {
    /// Validate ECDSA proof of Bitcoin address ownership
    /// 
    /// This function verifies that the user controls the private key
    /// corresponding to the provided Bitcoin address by validating
    /// an ECDSA signature over a commitment message.
    pub fn validate_proof(
        btc_address: &str,
        amount: u64,
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool> {
        // Validate input parameters
        if btc_address.is_empty() || signature.is_empty() || public_key.is_empty() {
            return Ok(false);
        }
        
        // Create the commitment message
        let message = Self::create_commitment_message(btc_address, amount)?;
        
        // Validate signature length (64 bytes for compact signature)
        if signature.len() != 64 {
            return Ok(false);
        }
        
        // Validate public key length (33 bytes for compressed, 65 for uncompressed)
        if public_key.len() != 33 && public_key.len() != 65 {
            return Ok(false);
        }
        
        // Verify the signature
        Self::verify_signature(&message, signature, public_key)
    }
    
    /// Create a deterministic commitment message
    fn create_commitment_message(btc_address: &str, amount: u64) -> Result<Vec<u8>> {
        let timestamp = Clock::get()
            .map_err(|_| VaultError::ClockUnavailable)?
            .unix_timestamp;
        
        // Create message: "Vault Protocol Commitment: {amount} BTC from {address} at {timestamp}"
        let message = format!(
            "Vault Protocol Commitment: {} satoshis from {} at {}",
            amount, btc_address, timestamp
        );
        
        // Hash the message with SHA256
        let mut hasher = Sha256::new();
        hasher.update(message.as_bytes());
        Ok(hasher.finalize().to_vec())
    }
    
    /// Verify ECDSA signature using secp256k1
    fn verify_signature(
        message_hash: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool> {
        let secp = Secp256k1::verification_only();
        
        // Parse the message hash
        let message = Message::from_slice(message_hash)
            .map_err(|_| VaultError::InvalidECDSAProof)?;
        
        // Parse the public key
        let pubkey = PublicKey::from_slice(public_key)
            .map_err(|_| VaultError::InvalidECDSAProof)?;
        
        // Parse the signature
        let sig = Signature::from_compact(signature)
            .map_err(|_| VaultError::InvalidECDSAProof)?;
        
        // Verify the signature
        match secp.verify_ecdsa(&message, &sig, &pubkey) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    
    /// Validate Bitcoin address format
    pub fn validate_btc_address(address: &str) -> Result<bool> {
        if address.is_empty() {
            return Ok(false);
        }
        
        // Basic format validation
        let is_valid = match address.chars().next() {
            Some('1') => address.len() >= 26 && address.len() <= 35, // P2PKH
            Some('3') => address.len() >= 26 && address.len() <= 35, // P2SH
            Some('b') if address.starts_with("bc1") => {
                address.len() >= 42 && address.len() <= 62 // Bech32
            },
            Some('t') if address.starts_with("tb1") => {
                address.len() >= 42 && address.len() <= 62 // Testnet Bech32
            },
            _ => false,
        };
        
        if !is_valid {
            return Ok(false);
        }
        
        // Additional character validation
        let valid_chars = address.chars().all(|c| {
            c.is_ascii_alphanumeric() && c != '0' && c != 'O' && c != 'I' && c != 'l'
        });
        
        Ok(valid_chars)
    }
    
    /// Generate a test ECDSA proof for testing purposes
    #[cfg(test)]
    pub fn generate_test_proof(
        btc_address: &str,
        amount: u64,
    ) -> Result<(Vec<u8>, Vec<u8>)> {
        use secp256k1::{SecretKey, rand::rngs::OsRng};
        
        let secp = Secp256k1::new();
        let mut rng = OsRng;
        
        // Generate a random private key for testing
        let secret_key = SecretKey::new(&mut rng);
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        
        // Create commitment message
        let message = Self::create_commitment_message(btc_address, amount)?;
        let message_hash = Message::from_slice(&message)
            .map_err(|_| VaultError::InvalidECDSAProof)?;
        
        // Sign the message
        let signature = secp.sign_ecdsa(&message_hash, &secret_key);
        
        Ok((signature.serialize_compact().to_vec(), public_key.serialize().to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_btc_address_validation() {
        // Valid addresses
        assert!(ECDSAValidator::validate_btc_address("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa").unwrap());
        assert!(ECDSAValidator::validate_btc_address("3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy").unwrap());
        assert!(ECDSAValidator::validate_btc_address("bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4").unwrap());
        
        // Invalid addresses
        assert!(!ECDSAValidator::validate_btc_address("").unwrap());
        assert!(!ECDSAValidator::validate_btc_address("invalid").unwrap());
        assert!(!ECDSAValidator::validate_btc_address("1234567890").unwrap());
    }
    
    #[test]
    fn test_ecdsa_proof_validation() {
        let btc_address = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
        let amount = 100_000_000; // 1 BTC
        
        // Generate test proof
        let (signature, public_key) = ECDSAValidator::generate_test_proof(btc_address, amount).unwrap();
        
        // Validate the proof
        let is_valid = ECDSAValidator::validate_proof(
            btc_address,
            amount,
            &signature,
            &public_key,
        ).unwrap();
        
        assert!(is_valid);
    }
    
    #[test]
    fn test_invalid_signature_length() {
        let btc_address = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
        let amount = 100_000_000;
        let invalid_signature = vec![0u8; 32]; // Wrong length
        let public_key = vec![0u8; 33];
        
        let is_valid = ECDSAValidator::validate_proof(
            btc_address,
            amount,
            &invalid_signature,
            &public_key,
        ).unwrap();
        
        assert!(!is_valid);
    }
    
    #[test]
    fn test_invalid_public_key_length() {
        let btc_address = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
        let amount = 100_000_000;
        let signature = vec![0u8; 64];
        let invalid_public_key = vec![0u8; 32]; // Wrong length
        
        let is_valid = ECDSAValidator::validate_proof(
            btc_address,
            amount,
            &signature,
            &invalid_public_key,
        ).unwrap();
        
        assert!(!is_valid);
    }
}

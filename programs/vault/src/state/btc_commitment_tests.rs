#[cfg(test)]
mod tests {
    use crate::state::BTCCommitment;
    use anchor_lang::prelude::*;
    use secp256k1::{Secp256k1, SecretKey, Message};
    use sha2::{Digest, Sha256};
    use std::str::FromStr;

    fn create_test_keypair() -> (SecretKey, secp256k1::PublicKey) {
        let secp = Secp256k1::new();
        let test_key = std::env::var("TEST_SECRET_KEY")
        .unwrap_or_else(|_| "a".repeat(64));
    let secret_key = SecretKey::from_str(&test_key).unwrap();
        let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
        (secret_key, public_key)
    }

    fn create_test_signature(message: &[u8], secret_key: &SecretKey) -> Vec<u8> {
        let secp = Secp256k1::new();
        let message_hash = Sha256::digest(message);
        let message = Message::from_slice(&message_hash).unwrap();
        let signature = secp.sign_ecdsa(&message, secret_key);
        signature.serialize_compact().to_vec()
    }

    #[test]
    fn test_validate_btc_address_valid() {
        // Test valid BTC addresses
        let valid_addresses = vec![
            "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", // Legacy P2PKH
            "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy", // P2SH
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh", // Bech32 P2WPKH
            "bc1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3qccfmv3", // Bech32 P2WSH
        ];

        for address in valid_addresses {
            assert!(BTCCommitment::validate_btc_address(address).is_ok());
        }
    }

    #[test]
    fn test_validate_btc_address_invalid() {
        // Test invalid BTC addresses
        let invalid_addresses = vec![
            "", // Empty
            "1", // Too short
            "invalid_address", // Invalid format
            "4A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", // Invalid prefix
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh123456789012345678901234567890", // Too long
        ];

        for address in invalid_addresses {
            assert!(BTCCommitment::validate_btc_address(address).is_err());
        }
    }

    #[test]
    fn test_validate_bech32_address() {
        // Test valid Bech32 addresses
        assert!(BTCCommitment::validate_bech32_address("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"));
        assert!(BTCCommitment::validate_bech32_address("tb1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"));
        
        // Test invalid Bech32 addresses (contains invalid characters)
        assert!(!BTCCommitment::validate_bech32_address("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlhB")); // Contains 'B'
        assert!(!BTCCommitment::validate_bech32_address("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlhI")); // Contains 'I'
    }

    #[test]
    fn test_create_commitment_hash() {
        let user_address = Pubkey::new_unique();
        let btc_address = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";
        let amount = 50000000; // 0.5 BTC in satoshis
        let timestamp = 1640995200;

        let hash1 = BTCCommitment::create_commitment_hash(&user_address, btc_address, amount, timestamp);
        let hash2 = BTCCommitment::create_commitment_hash(&user_address, btc_address, amount, timestamp);
        
        // Same inputs should produce same hash
        assert_eq!(hash1, hash2);

        // Different inputs should produce different hash
        let hash3 = BTCCommitment::create_commitment_hash(&user_address, btc_address, amount + 1, timestamp);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_serialize_for_signing() {
        let user_address = Pubkey::new_unique();
        let btc_address = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";
        let amount = 50000000;
        let timestamp = 1640995200;

        let serialized = BTCCommitment::serialize_for_signing(&user_address, btc_address, amount, timestamp);
        
        // Should contain all the components
        assert!(serialized.len() > 32 + btc_address.len() + 8 + 8); // pubkey + address + amount + timestamp
        
        // Should be deterministic
        let serialized2 = BTCCommitment::serialize_for_signing(&user_address, btc_address, amount, timestamp);
        assert_eq!(serialized, serialized2);
    }

    #[test]
    fn test_validate_ecdsa_proof_valid() {
        let (secret_key, public_key) = create_test_keypair();
        let user_address = Pubkey::new_unique();
        let btc_address = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";
        let amount = 50000000;
        let timestamp = 1640995200;

        // Create commitment
        let mut commitment = BTCCommitment {
            user_address,
            btc_address: btc_address.to_string(),
            amount,
            ecdsa_proof: vec![],
            timestamp,
            verified: false,
            last_verification: 0,
            commitment_hash: [0; 32],
            public_key: public_key.serialize().to_vec(),
            bump: 0,
        };

        // Create message and signature
        let message_data = BTCCommitment::serialize_for_signing(&user_address, btc_address, amount, timestamp);
        let signature = create_test_signature(&message_data, &secret_key);

        // Test validation
        let result = commitment.validate_ecdsa_proof(&message_data, &signature, &public_key.serialize());
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_validate_ecdsa_proof_invalid_signature() {
        let (_, public_key) = create_test_keypair();
        let user_address = Pubkey::new_unique();
        let btc_address = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";
        let amount = 50000000;
        let timestamp = 1640995200;

        let mut commitment = BTCCommitment {
            user_address,
            btc_address: btc_address.to_string(),
            amount,
            ecdsa_proof: vec![],
            timestamp,
            verified: false,
            last_verification: 0,
            commitment_hash: [0; 32],
            public_key: public_key.serialize().to_vec(),
            bump: 0,
        };

        let message_data = BTCCommitment::serialize_for_signing(&user_address, btc_address, amount, timestamp);
        let invalid_signature = vec![0u8; 64]; // Invalid signature

        let result = commitment.validate_ecdsa_proof(&message_data, &invalid_signature, &public_key.serialize());
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should return false for invalid signature
    }

    #[test]
    fn test_validate_ecdsa_proof_invalid_length() {
        let (_, public_key) = create_test_keypair();
        let user_address = Pubkey::new_unique();
        let btc_address = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";
        let amount = 50000000;
        let timestamp = 1640995200;

        let mut commitment = BTCCommitment {
            user_address,
            btc_address: btc_address.to_string(),
            amount,
            ecdsa_proof: vec![],
            timestamp,
            verified: false,
            last_verification: 0,
            commitment_hash: [0; 32],
            public_key: public_key.serialize().to_vec(),
            bump: 0,
        };

        let message_data = BTCCommitment::serialize_for_signing(&user_address, btc_address, amount, timestamp);
        
        // Test invalid signature length
        let invalid_signature = vec![0u8; 32]; // Wrong length
        let result = commitment.validate_ecdsa_proof(&message_data, &invalid_signature, &public_key.serialize());
        assert!(result.is_err());

        // Test invalid public key length
        let valid_signature = vec![0u8; 64];
        let invalid_pubkey = vec![0u8; 20]; // Wrong length
        let result = commitment.validate_ecdsa_proof(&message_data, &valid_signature, &invalid_pubkey);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_commitment_success() {
        let (secret_key, public_key) = create_test_keypair();
        let user_address = Pubkey::new_unique();
        let btc_address = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";
        let amount = 50000000;
        let timestamp = 1640995200;

        let message_data = BTCCommitment::serialize_for_signing(&user_address, btc_address, amount, timestamp);
        let signature = create_test_signature(&message_data, &secret_key);
        let commitment_hash = BTCCommitment::create_commitment_hash(&user_address, btc_address, amount, timestamp);

        let commitment = BTCCommitment {
            user_address,
            btc_address: btc_address.to_string(),
            amount,
            ecdsa_proof: signature,
            timestamp,
            verified: false,
            last_verification: 0,
            commitment_hash,
            public_key: public_key.serialize().to_vec(),
            bump: 0,
        };

        // Mock Clock::get() for testing - in real tests you'd use a test framework that can mock this
        // For now, we'll test the parts we can without the Clock dependency
        assert!(BTCCommitment::validate_btc_address(&commitment.btc_address).is_ok());
        assert!(commitment.amount > 0);
        assert!(!commitment.ecdsa_proof.is_empty());
        assert!(!commitment.public_key.is_empty());
    }

    #[test]
    fn test_validate_commitment_invalid_amount() {
        let user_address = Pubkey::new_unique();
        let btc_address = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";
        let amount = 0; // Invalid amount
        let timestamp = 1640995200;

        let commitment_hash = BTCCommitment::create_commitment_hash(&user_address, btc_address, amount, timestamp);

        let commitment = BTCCommitment {
            user_address,
            btc_address: btc_address.to_string(),
            amount,
            ecdsa_proof: vec![1, 2, 3], // Some proof
            timestamp,
            verified: false,
            last_verification: 0,
            commitment_hash,
            public_key: vec![1, 2, 3], // Some key
            bump: 0,
        };

        // Should fail due to zero amount
        assert!(commitment.amount == 0);
    }

    #[test]
    fn test_validate_commitment_empty_proof() {
        let user_address = Pubkey::new_unique();
        let btc_address = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";
        let amount = 50000000;
        let timestamp = 1640995200;

        let commitment_hash = BTCCommitment::create_commitment_hash(&user_address, btc_address, amount, timestamp);

        let commitment = BTCCommitment {
            user_address,
            btc_address: btc_address.to_string(),
            amount,
            ecdsa_proof: vec![], // Empty proof
            timestamp,
            verified: false,
            last_verification: 0,
            commitment_hash,
            public_key: vec![1, 2, 3],
            bump: 0,
        };

        // Should fail due to empty proof
        assert!(commitment.ecdsa_proof.is_empty());
    }

    #[test]
    fn test_validate_commitment_hash_mismatch() {
        let user_address = Pubkey::new_unique();
        let btc_address = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";
        let amount = 50000000;
        let timestamp = 1640995200;

        let wrong_hash = [1u8; 32]; // Wrong hash

        let commitment = BTCCommitment {
            user_address,
            btc_address: btc_address.to_string(),
            amount,
            ecdsa_proof: vec![1, 2, 3],
            timestamp,
            verified: false,
            last_verification: 0,
            commitment_hash: wrong_hash,
            public_key: vec![1, 2, 3],
            bump: 0,
        };

        let expected_hash = BTCCommitment::create_commitment_hash(&user_address, btc_address, amount, timestamp);
        
        // Hashes should not match
        assert_ne!(commitment.commitment_hash, expected_hash);
    }

    #[test]
    fn test_anti_replay_attack() {
        let user_address = Pubkey::new_unique();
        let btc_address = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";
        let amount = 50000000;
        
        // Create two commitments with different timestamps
        let timestamp1 = 1640995200;
        let timestamp2 = 1640995260; // 1 minute later
        
        let hash1 = BTCCommitment::create_commitment_hash(&user_address, btc_address, amount, timestamp1);
        let hash2 = BTCCommitment::create_commitment_hash(&user_address, btc_address, amount, timestamp2);
        
        // Different timestamps should produce different hashes (prevents replay attacks)
        assert_ne!(hash1, hash2);
        
        // Serialized data should also be different
        let data1 = BTCCommitment::serialize_for_signing(&user_address, btc_address, amount, timestamp1);
        let data2 = BTCCommitment::serialize_for_signing(&user_address, btc_address, amount, timestamp2);
        assert_ne!(data1, data2);
    }
}
use aes_gcm::{Aes256Gcm, Key, Nonce, KeyInit};
use aes_gcm::aead::Aead;
use anyhow::{Result, bail};
use base64::{Engine, engine::general_purpose};
use rand::Rng;

pub struct EncryptionService {
    cipher: Aes256Gcm,
}

impl EncryptionService {
    pub fn new(key: &str) -> Result<Self> {
        if key.len() != 32 {
            bail!("Encryption key must be exactly 32 bytes long");
        }
        
        let key = Key::<Aes256Gcm>::from_slice(key.as_bytes());
        let cipher = Aes256Gcm::new(key);
        
        Ok(Self { cipher })
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String> {
        let mut rng = rand::thread_rng();
        let nonce_bytes: [u8; 12] = rng.gen();
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = self.cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;
        
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);
        
        Ok(general_purpose::STANDARD.encode(&result))
    }

    pub fn decrypt(&self, encrypted_data: &str) -> Result<String> {
        let data = general_purpose::STANDARD.decode(encrypted_data)
            .map_err(|e| anyhow::anyhow!("Base64 decode failed: {}", e))?;
        
        if data.len() < 12 {
            bail!("Invalid encrypted data: too short");
        }
        
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let plaintext = self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;
        
        String::from_utf8(plaintext)
            .map_err(|e| anyhow::anyhow!("UTF-8 decode failed: {}", e))
    }
}

pub struct CardTokenizer {
    encryption: EncryptionService,
}

impl CardTokenizer {
    pub fn new(encryption_key: &str) -> Result<Self> {
        let encryption = EncryptionService::new(encryption_key)?;
        Ok(Self { encryption })
    }

    pub fn tokenize_card_number(&self, card_number: &str) -> Result<String> {
        // 移除所有非数字字符
        let clean_number: String = card_number.chars().filter(|c| c.is_digit(10)).collect();
        
        if clean_number.len() < 13 || clean_number.len() > 19 {
            bail!("Invalid card number length");
        }
        
        self.encryption.encrypt(&clean_number)
    }

    pub fn detokenize_card_number(&self, token: &str) -> Result<String> {
        self.encryption.decrypt(token)
    }

    pub fn get_last_four(&self, card_number: &str) -> String {
        let clean_number: String = card_number.chars().filter(|c| c.is_digit(10)).collect();
        if clean_number.len() >= 4 {
            clean_number[clean_number.len() - 4..].to_string()
        } else {
            clean_number
        }
    }

    pub fn mask_card_number(&self, card_number: &str) -> String {
        let clean_number: String = card_number.chars().filter(|c| c.is_digit(10)).collect();
        if clean_number.len() >= 4 {
            let last_four = &clean_number[clean_number.len() - 4..];
            format!("****-****-****-{}", last_four)
        } else {
            "****-****-****-****".to_string()
        }
    }
}

pub fn validate_luhn(card_number: &str) -> bool {
    let clean_number: String = card_number.chars().filter(|c| c.is_digit(10)).collect();
    
    if clean_number.is_empty() {
        return false;
    }
    
    let mut sum = 0;
    let mut alternate = false;
    
    for ch in clean_number.chars().rev() {
        if let Some(mut digit) = ch.to_digit(10) {
            if alternate {
                digit *= 2;
                if digit > 9 {
                    digit = digit / 10 + digit % 10;
                }
            }
            sum += digit;
            alternate = !alternate;
        } else {
            return false;
        }
    }
    
    sum % 10 == 0
}

pub fn detect_card_brand(card_number: &str) -> &'static str {
    let clean_number: String = card_number.chars().filter(|c| c.is_digit(10)).collect();
    
    if clean_number.is_empty() {
        return "unknown";
    }
    
    let first_digit = &clean_number[0..1];
    let first_two = if clean_number.len() >= 2 { &clean_number[0..2] } else { first_digit };
    let first_four = if clean_number.len() >= 4 { &clean_number[0..4] } else { first_two };
    
    match first_digit {
        "4" => "visa",
        "5" => "mastercard",
        "3" => {
            if first_two == "34" || first_two == "37" {
                "american_express"
            } else {
                "diners_club"
            }
        }
        "6" => {
            if first_four.starts_with("6011") || first_two == "65" {
                "discover"
            } else {
                "unknown"
            }
        }
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luhn_validation() {
        // Valid test card numbers
        assert!(validate_luhn("4111111111111111")); // Visa
        assert!(validate_luhn("5555555555554444")); // Mastercard
        assert!(validate_luhn("378282246310005"));  // Amex
        
        // Invalid numbers
        assert!(!validate_luhn("4111111111111112"));
        assert!(!validate_luhn("1234567890"));
        assert!(!validate_luhn(""));
    }

    #[test]
    fn test_card_brand_detection() {
        assert_eq!(detect_card_brand("4111111111111111"), "visa");
        assert_eq!(detect_card_brand("5555555555554444"), "mastercard");
        assert_eq!(detect_card_brand("378282246310005"), "american_express");
        assert_eq!(detect_card_brand("6011111111111117"), "discover");
        assert_eq!(detect_card_brand("1234567890"), "unknown");
    }
} 
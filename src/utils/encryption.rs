use crate::models::stream_urls::StreamUrls;
use aes::Aes128;
use base64::{engine::general_purpose, Engine as _};
use cbc::{
    cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit},
    Decryptor,
};
use std::fmt;

// AES decryption constants from Gaana
const AES_KEY: &[u8] = b"g@1n!(f1#r.0$)&%";
const AES_IV: &[u8] = b"asd!@#!@#@!12312";

type Aes128CbcDec = Decryptor<Aes128>;

#[derive(Debug)]
pub enum DecryptionError {
    Base64Error(base64::DecodeError),
    DecryptionFailed(String),
    Utf8Error(std::string::FromUtf8Error),
}

impl fmt::Display for DecryptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecryptionError::Base64Error(e) => write!(f, "Base64 decode error: {}", e),
            DecryptionError::DecryptionFailed(msg) => write!(f, "Decryption failed: {}", msg),
            DecryptionError::Utf8Error(e) => write!(f, "UTF-8 conversion error: {}", e),
        }
    }
}

impl std::error::Error for DecryptionError {}

/// Decrypt Gaana stream URLs using AES decryption
pub fn decrypt_stream_url(encrypted_url: &str) -> StreamUrls {
    match decrypt_link(encrypted_url) {
        Ok(decrypted_url) => {
            // Generate different quality URLs from the decrypted base URL
            generate_quality_variants(&decrypted_url)
        }
        Err(_) => {
            // Return placeholder URLs if decryption fails
            generate_fallback_urls()
        }
    }
}

/// Decrypt an encrypted link using AES CBC mode
pub fn decrypt_link(encrypted_link: &str) -> Result<String, DecryptionError> {
    // Decode base64
    let encrypted_data = general_purpose::STANDARD
        .decode(encrypted_link)
        .map_err(DecryptionError::Base64Error)?;

    // Create AES CBC decryptor
    let cipher = Aes128CbcDec::new(AES_KEY.into(), AES_IV.into());

    // Decrypt the data
    let mut decrypted_data = encrypted_data.clone();
    let decrypted_bytes = cipher
        .decrypt_padded_mut::<Pkcs7>(&mut decrypted_data)
        .map_err(|_| DecryptionError::DecryptionFailed("AES decryption failed".to_string()))?;

    // Convert to string
    let decrypted_string =
        String::from_utf8(decrypted_bytes.to_vec()).map_err(DecryptionError::Utf8Error)?;

    Ok(decrypted_string)
}

/// Generate quality variants from the decrypted URL
fn generate_quality_variants(base_url: &str) -> StreamUrls {
    // Generate different quality URLs by modifying bitrate parameters
    // Based on Python code: base 64.mp4, then replace for different qualities
    let vhq_url = base_url.replace("64.mp4", "320.mp4");
    let hq_url = base_url.replace("64.mp4", "128.mp4");
    let mq_url = base_url.to_string(); // Medium quality is the base URL (64.mp4)
    let lq_url = base_url.replace("64.mp4", "16.mp4");

    StreamUrls::new(Some(vhq_url), Some(hq_url), Some(mq_url), Some(lq_url))
}

/// Generate fallback URLs when decryption fails
fn generate_fallback_urls() -> StreamUrls {
    let placeholder_url = "https://stream-cdn.gaana.com/placeholder.m3u8".to_string();

    StreamUrls::new(
        Some(placeholder_url.clone()),
        Some(placeholder_url.clone()),
        Some(placeholder_url.clone()),
        Some(placeholder_url),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_quality_variants() {
        let base_url = "https://stream-cdn.gaana.com/track/128.mp4";
        let variants = generate_quality_variants(base_url);

        assert!(variants.very_high_quality.is_some());
        assert!(variants.high_quality.is_some());
        assert!(variants.medium_quality.is_some());
        assert!(variants.low_quality.is_some());
    }

    #[test]
    fn test_fallback_urls() {
        let fallback = generate_fallback_urls();

        assert!(fallback.very_high_quality.is_some());
        assert!(fallback.high_quality.is_some());
        assert!(fallback.medium_quality.is_some());
        assert!(fallback.low_quality.is_some());
    }

    #[test]
    fn test_decrypt_link_invalid_base64() {
        let invalid_b64 = "invalid_base64!@#";
        let result = decrypt_link(invalid_b64);
        assert!(result.is_err());
    }
}

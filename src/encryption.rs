use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use chacha20poly1305::{
    Key as ChaChaKey, XChaCha20Poly1305, XNonce,
    aead::{Aead, KeyInit, Payload as AeadPayload},
};
use dotenv::dotenv;
use scrypt;
use std::env;

/// Converts encrypted data to a base64-encoded string for storage or transmission
pub fn encrypt_to_string(
    plaintext_data: &[u8],
    master_password: Option<&[u8]>,
) -> Result<String, anyhow::Error> {
    let encrypted = encrypt_data(plaintext_data, master_password)?;
    Ok(BASE64.encode(encrypted.ciphertext))
}

/// Encrypts plaintext data using a master password.
fn encrypt_data(
    plaintext_data: &[u8],
    master_password: Option<&[u8]>,
) -> Result<EncryptedPayload, anyhow::Error> {
    let salt = get_salt_from_env()?;
    let log_n = SCRYPT_LOG_N;

    // STEP 1: KEY DERIVATION
    let derived_key_array: [u8; KEY_SIZE] = derive_key(master_password, &salt, log_n)?;
    let chacha_key = ChaChaKey::from_slice(&derived_key_array);

    // STEP 2: CIPHER INITIALIZATION
    let cipher = XChaCha20Poly1305::new(chacha_key);

    // STEP 3: NONCE GENERATION
    let nonce_bytes = [0u8; NONCE_SIZE];
    // OsRng.fill_bytes(&mut nonce_bytes);
    let nonce_obj = XNonce::from_slice(&nonce_bytes);

    // STEP 4: PAYLOAD CONSTRUCTION (for encryption)
    let aad = [KeySecurity::High as u8];
    let aead_payload = AeadPayload {
        msg: plaintext_data,
        aad: &aad,
    };

    // STEP 5: ENCRYPTION
    let ciphertext = cipher
        .encrypt(nonce_obj, aead_payload)
        .expect("encryption failure!");

    Ok(EncryptedPayload {
        ciphertext,
        salt,
        nonce: nonce_bytes,
        log_n,
    })
}

fn get_salt_from_env() -> Result<[u8; 32], anyhow::Error> {
    dotenv().ok();
    let env_salt = env::var("SALT").expect("SALT must be set in env file");
    let salt_bytes = env_salt.as_bytes();
    let mut salt = [0u8; 32];
    if salt_bytes.len() >= 32 {
        salt.copy_from_slice(&salt_bytes[0..32]);
    } else {
        salt[..salt_bytes.len()].copy_from_slice(salt_bytes);
        // remaining bytes remain 0
    }
    Ok(salt)
}

fn derive_key(
    password: Option<&[u8]>,
    salt: &[u8; SALT_SIZE],
    log_n: u8,
) -> Result<[u8; KEY_SIZE], anyhow::Error> {
    let key_len_nz = core::num::NonZeroUsize::new(KEY_SIZE)
        .expect("KEY_SIZE must be non-zero for scrypt params");
    let params = scrypt::Params::new(log_n, SCRYPT_R, SCRYPT_P, key_len_nz.get())?;

    let mut key = [0u8; KEY_SIZE];
    if password.is_some() {
        scrypt::scrypt(password.unwrap(), salt, &params, &mut key)?;
    } else {
        scrypt::scrypt("".as_bytes(), salt, &params, &mut key)?;
    }
    Ok(key)
}

/// Convenience function to decrypt data from String and convert it to a String.
pub fn decrypt_from_string(
    encrypted_base64: &str,
    master_password: Option<&[u8]>,
) -> Result<String, anyhow::Error> {
    let salt = get_salt_from_env()?;
    // Decode the base64 string back into bytes
    let ciphertext = BASE64
        .decode(encrypted_base64)
        .map_err(|_| anyhow::Error::msg("Invalid base64 encoding"))?;

    let nonce_bytes = [0u8; NONCE_SIZE];
    let encrypted_payload = EncryptedPayload {
        ciphertext,
        salt,
        nonce: nonce_bytes,
        log_n: SCRYPT_LOG_N,
    };
    decrypt_password_string(&encrypted_payload, master_password)
}

/// Convenience function to decrypt data and convert it to a String.
fn decrypt_password_string(
    encrypted_payload: &EncryptedPayload,
    master_password: Option<&[u8]>,
) -> Result<String, anyhow::Error> {
    let decrypted_bytes = decrypt_data(encrypted_payload, master_password)?;
    String::from_utf8(decrypted_bytes).map_err(anyhow::Error::new)
}

/// Decrypts an `EncryptedPayload` to recover the original plaintext data as bytes.
fn decrypt_data(
    encrypted_payload: &EncryptedPayload,
    master_password: Option<&[u8]>,
) -> Result<Vec<u8>, anyhow::Error> {
    // STEP 1: KEY DERIVATION
    let derived_key_array: [u8; KEY_SIZE] = derive_key(
        master_password,
        &encrypted_payload.salt,
        encrypted_payload.log_n,
    )?;
    let chacha_key = ChaChaKey::from_slice(&derived_key_array);

    // STEP 2: CIPHER INITIALIZATION
    let cipher = XChaCha20Poly1305::new(chacha_key);

    // STEP 3: PAYLOAD CONSTRUCTION (for decryption)
    let aad = [KeySecurity::High as u8];
    let aead_payload = AeadPayload {
        msg: encrypted_payload.ciphertext.as_slice(),
        aad: &aad,
    };
    let nonce_obj = XNonce::from_slice(&encrypted_payload.nonce);

    // STEP 4: DECRYPTION
    let decrypted_bytes = cipher
        .decrypt(nonce_obj, aead_payload)
        .expect("decryption failed");

    Ok(decrypted_bytes)
}

#[derive(Debug, Clone)]
struct EncryptedPayload {
    ciphertext: Vec<u8>,
    salt: [u8; SALT_SIZE],   // Salt used for key derivation
    nonce: [u8; NONCE_SIZE], // Nonce used for encryption
    log_n: u8,               // Scrypt log_n parameter used
}

const KEY_SIZE: usize = 32; // For XChaCha20Poly1305 derived key
const NONCE_SIZE: usize = 24; // For XChaCha20 nonce
const SALT_SIZE: usize = 32; // Salt length for scrypt

// Scrypt parameters
const SCRYPT_LOG_N: u8 = 15; // N = 2^log_n. 15 => N=32768. Adjust for security/performance.
const SCRYPT_R: u32 = 8; // Scrypt 'r' parameter (memory factor)
const SCRYPT_P: u32 = 1; // Scrypt 'p' parameter (parallelization factor)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum KeySecurity {
    // Low = 0,
    // Medium = 1,
    High = 2,
}

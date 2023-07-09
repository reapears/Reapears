//! Token impls

use serde::Deserialize;

const TOKEN_LENGTH: usize = 96;
const CODE_LENGTH: usize = 6;

/// Default to blake3 hash length 32bytes
const HASH_LENGTH: usize = blake3::OUT_LEN; // 32
pub type TokenHash = [u8; HASH_LENGTH];

/// Hashes and returns the hash of the token
#[must_use]
pub fn hash_token(token: &[u8]) -> TokenHash {
    secure::hash(token)
}

/// Deserializes token query param from request url
#[derive(Debug, Clone, Deserialize)]
pub struct TokenConfirm {
    pub token: String,
}

/// Toke
#[derive(Clone)]
pub struct Token {
    pub plaintext: String,
    pub hash: TokenHash,
}

impl Default for Token {
    /// Generates a new token using the default length of `TOKEN_LENGTH`
    fn default() -> Self {
        secure::generate_token(TOKEN_LENGTH)
    }
}

impl Token {
    /// Generates a new token of a given len
    #[must_use]
    pub fn generate(len: usize) -> Self {
        secure::generate_token(len)
    }

    /// Generates a new session token
    #[must_use]
    pub fn new_session() -> Self {
        secure::generate_token(TOKEN_LENGTH)
    }

    /// Generates a new new verification code
    #[must_use]
    pub fn new_code() -> Self {
        secure::generate_code(CODE_LENGTH)
    }

    /// Returns a token and it's hash
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn into_parts(self) -> (String, TokenHash) {
        (self.plaintext, self.hash)
    }
}

/// Verify client's token against the saved hash on the database
///
/// # Errors
///
/// Return an error if failed to parse the `hash`
#[must_use]
pub fn verify_token(hash: TokenHash, token: &[u8]) -> bool {
    blake3::Hash::from(hash) == blake3::hash(token)
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Token{...}")
    }
}

mod secure {
    use super::{Token, TokenHash};
    use rand::{distributions::Uniform, rngs::OsRng, Rng};

    pub fn generate_token(len: usize) -> Token {
        let plaintext = generate_alphanumeric_string(len);
        let hash = hash(plaintext.as_bytes());
        Token { plaintext, hash }
    }

    pub fn generate_code(len: usize) -> Token {
        let plaintext = generate_numeric(len);
        let hash = hash(plaintext.as_bytes());
        Token { plaintext, hash }
    }

    pub fn hash(plaintext: &[u8]) -> TokenHash {
        blake3::hash(plaintext).as_bytes().to_owned()
    }

    pub fn generate_alphanumeric_string(len: usize) -> String {
        const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        OsRng
            .sample_iter(Uniform::from(0..CHARS.len()))
            .map(|idx| CHARS[idx] as char)
            .take(len)
            .collect()
    }

    pub fn generate_numeric(len: usize) -> String {
        const CHARS: &[u8] = b"0123456789";
        OsRng
            .sample_iter(Uniform::from(0..CHARS.len()))
            .map(|idx| CHARS[idx] as char)
            .take(len)
            .collect()
    }
}

mod tests {}

//! Nominal types, wrapping more basic types, that encapsulate a specific
//! piece of data, so they don't get mixed up. For example, "user public key"
//! and "session public key"

pub mod hashed_password_verifier;
pub mod password_key;
pub mod password_verifier;
pub mod user_input;
pub mod user_keypair;
pub mod verification_token;

use super::*;

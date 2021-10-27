//! Implements BLS signatures as specified in https://crypto.stanford.edu/~dabo/pubs/papers/BLSmultisig.html.

mod secret;
pub use secret::PrivateKey;

mod public;
pub use public::PublicKey;

mod signature;
pub use signature::Signature;

#[cfg(feture = "getrandom")] mod cache;
#[cfg(feture = "getrandom")] pub use cache::PublicKeyCache;

#[cfg(feture = "getrandom")] mod batch;
#[cfg(feture = "getrandom")] pub use batch::Batch;

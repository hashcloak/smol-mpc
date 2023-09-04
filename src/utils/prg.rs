//! Implementation of a PRG based on AES-CTR. 
//! 
//! The idea is that a pseudo-random block of bits is generated by computing
//!    $$\textsf{Block} = \textsf{AES}(\textsf{Seed}, \textsf{CurrentCounter})$$
//! where $\textsf{CurrentCounter} = \textsf{Nonce} \Vert \textsf{Counter}$, and the counter
//! is increased when a new block is generated.
//! 
//! This implementation is based on the one used in [Secure Computation Library].
//! 
//! [Secure Computation Library]: https://github.com/anderspkd/secure-computation-library/blob/master/include/scl/util/prg.h

use aes::cipher::{KeyIvInit, StreamCipher};
use std::vec;

type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;

/// Defines a pseudo-random number generator.
pub struct Prg {
    seed: Vec<u8>,
    counter: u64,
}

impl Prg {
    const PRG_NONCE: u64 = 0x0123456789ABCDEF;
    const PRG_INITIAL_COUNTER: u64 = 0;

    // All the lengths are in bytes
    const KEY_LEN: usize = 16;
    const IV_LEN: usize = 16;
    const BLOCK_LEN: usize = 16;

    /// Creates a new PRG. 
    /// 
    /// The seed can be provided or generated by default
    /// filled with zeros. If a seed is provided, then there are two cases. If
    /// the key is longer than the key length for the encryption scheme, it is
    /// cropped to fit in the specification. If the key is shorter than the
    /// expected key, the key is padded with zeros. The key will be divided in
    /// two halves to generate the encryption key and the initialization vector
    /// for the CTR mode.
    pub fn new(seed: Option<Vec<u8>>) -> Prg {
        let cropped_seed = if let Some(mut value_seed) = seed {
            if value_seed.len() > Self::KEY_LEN + Self::IV_LEN {
                value_seed[0..Self::KEY_LEN + Self::IV_LEN].to_vec()
            } else {
                let original_seed_length = value_seed.len();
                let mut appended_seed = Vec::new();
                appended_seed.append(&mut value_seed);
                appended_seed.append(&mut vec![
                    0;
                    Self::KEY_LEN + Self::IV_LEN - original_seed_length
                ]);
                appended_seed
            }
        } else {
            vec![0; Self::KEY_LEN + Self::IV_LEN]
        };

        let counter = Self::PRG_INITIAL_COUNTER;

        let mut prg = Prg {
            seed: cropped_seed,
            counter,
        };
        prg.init();
        prg
    }

    /// Initializes the PRG.
    pub fn init(&mut self) {
        self.counter = Self::PRG_INITIAL_COUNTER;
    }

    /// Resets the PRG.
    pub fn reset(&mut self) {
        self.init()
    }

    /// Returns the current state of the counter in the PRG.
    pub fn counter(&self) -> u64 {
        self.counter
    }

    /// Generates a stream of random bytes.
    /// 
    /// The method divides the seed into two halves: the first part will be used
    /// as the key for the AES encryption and the second part will be used as
    /// the initialization vector for the encryption.
    pub fn next(&mut self, n_bytes: usize) -> Vec<u8> {
        if n_bytes == 0 {
            return Vec::new();
        }

        // Compute the number of blocks needed
        let mut n_blocks = n_bytes / Self::BLOCK_LEN;
        if n_bytes % Self::BLOCK_LEN != 0 {
            n_blocks += 1;
        }

        let key = &self.seed[0..Self::KEY_LEN];
        let iv = &self.seed[Self::KEY_LEN..];

        let mut cipher = Aes128Ctr64LE::new(key.into(), iv.into());

        let mut out = Vec::new();
        for _ in 0..n_blocks {
            let mut buffer = [Self::PRG_NONCE.to_ne_bytes(), self.counter.to_ne_bytes()].concat();
            cipher.apply_keystream(&mut buffer);
            out.append(&mut buffer);

            self.counter += 1;
        }

        out[..n_bytes].to_vec()
    }
}

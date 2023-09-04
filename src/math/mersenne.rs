//! Implements all the functionalities for Mersenne fields.
//!
//! In multi-party computation protocols, it is common to use an underlying
//! algebraic structure in wich all the secure computations are performed. This
//! module defines all the basic functionalities to manipulate the Mersenne
//! fields used in the protocols supported by the library.
//!
//! The source code for `Mersenne61` was taken from [Secure Computation Library].
//!
//! [Secure Computation Library]: https://github.com/anderspkd/secure-computation-library/blob/master/src/scl/math/mersenne61.cc

use crate::utils::prg::Prg;

/// Defines an element in a Mersenne field $\mathbb{F}_p$ with $p = 2 ^ {61} - 1$.
#[derive(Clone)]
pub struct Mersenne61 {
    /// Value of the element. This value will belong to $\mathbb{F}_p$.
    pub value: u64,
}

/// Defines the operations over Mersenne fields elements.
pub trait MersenneField {
    /// Power of the Mersenne field. Mersenne fields are of the form
    /// $\mathbb{F}_p$ with $p = 2^n - 1$. This variable represents $n$.
    const POWER: u64;

    /// Order of the Mersenne field.
    const ORDER: u64;

    /// Creates an element in a Mersenne field.
    fn new(value: u64) -> Self;

    /// Computes the sum between two elements in a Mersenne field.
    fn add(&self, other: &Self) -> Self;

    /// Given a field element $a \in \mathbb{F}_p$, returns $-a$.
    fn negate(&self) -> Self;

    /// Computes the product of two elements in the Mersenne field.
    fn multiply(&self, other: &Self) -> Self;

    /// Given a field element $a \in \mathbb{F}_p$, returns $a^{-1}$.
    fn inverse(&self) -> Self;

    /// Computes the subtraction between two elements in the field.
    fn subtract(&self, other: &Self) -> Self;

    /// Generates a random element in the Mersenne field provided a
    /// pseudo-random generator.
    fn random(prg: &mut Prg) -> Self;

    /// Returns the value of the element in the Mersenne field.
    fn value(&self) -> u64;
}

impl MersenneField for Mersenne61 {
    const POWER: u64 = 61;
    const ORDER: u64 = (1 << Self::POWER) - 1;

    fn new(value: u64) -> Self {
        if value < Self::ORDER {
            Self { value }
        } else {
            // TODO: This is provisional while I find a way to do it in constant
            // time.
            Self {
                value: value % Self::ORDER,
            }
        }
    }

    fn value(&self) -> u64 {
        self.value
    }

    fn add(&self, other: &Self) -> Self {
        let sum = self.value + other.value;
        if sum >= Self::ORDER {
            Self {
                value: sum - Self::ORDER,
            }
        } else {
            Self { value: sum }
        }
    }

    fn subtract(&self, other: &Self) -> Self {
        self.add(&other.negate())
    }

    fn inverse(&self) -> Self {
        if self.value == 0 {
            panic!("You can not invert the zero element of a field.");
        }

        let mut k: i64 = 0;
        let mut new_k: i64 = 1;
        let mut r = Self::ORDER as i64;
        let mut new_r = self.value as i64;

        while new_r != 0 {
            let q = r / new_r;

            // Swaps and operates on k and new_k, and r and new_r
            swap_and_operate(&mut k, &mut new_k, q);
            swap_and_operate(&mut r, &mut new_r, q);
        }

        if k < 0 {
            k += Self::ORDER as i64;
        }

        Self { value: k as u64 }
    }

    fn multiply(&self, other: &Self) -> Self {
        let mult: u128 = (self.value as u128) * (other.value as u128);
        let mut a = mult >> Self::POWER;
        let mut b: u64 = mult as u64;

        a |= (b as u128) >> (Self::POWER as u128);
        b &= Self::ORDER;

        let a_wrap = Self { value: a as u64 };
        let b_wrap = Self { value: b };

        a_wrap.add(&b_wrap)
    }

    fn negate(&self) -> Self {
        if self.value != 0 {
            Self {
                value: Self::ORDER - self.value,
            }
        } else {
            self.clone()
        }
    }

    fn random(prg: &mut Prg) -> Self {
        let random_bytes = prg.next((u64::BITS / 8) as usize);
        let random_value = u64::from_ne_bytes(
            random_bytes
                .try_into()
                .expect("Expected a vector with 8 bytes"),
        );

        Self::new(random_value)
    }
}

fn swap_and_operate(a: &mut i64, b: &mut i64, q: i64) {
    let temp = *b;
    *b = *a - q * temp;
    *a = temp;
}

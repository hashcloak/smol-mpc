/// The source code for Mersenne61 was taken from
/// https://github.com/anderspkd/secure-computation-library/blob/master/src/scl/math/mersenne61.cc
use crate::utils::prg::Prg;

#[derive(Clone)]
pub struct Mersenne61 {
    pub value: u64,
}

pub trait MersenneField {
    const POWER: u64;
    const ORDER: u64;

    fn new(value: u64) -> Self;

    fn add(&self, other: &Self) -> Self;

    fn negate(&self) -> Self;

    fn multiply(&self, other: &Self) -> Self;

    fn inverse(&self) -> Self;

    fn subtract(&self, other: &Self) -> Self;

    fn random(prg: &mut Prg) -> Self;
}

impl MersenneField for Mersenne61 {
    const POWER: u64 = 61;
    const ORDER: u64 = (1 << Self::POWER) - 1;

    fn new(value: u64) -> Self {
        if value < Self::ORDER {
            Self { value }
        } else {
            Self {
                value: value - Self::ORDER,
            }
        }
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
        return self.add(&other.negate());
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
            k = k + (Self::ORDER as i64);
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

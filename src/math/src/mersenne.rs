
pub struct Mersenne61 {
}

pub trait MersenneField {
    const POWER: u64;

    fn add(&self, other: Self) -> Self;

    fn negate(&self) -> Self;

    fn multiply(&self, other: Self) -> Self;

    fn inverse(&self) -> Self;
}

impl MersenneField for Mersenne61 {
    const POWER: u64 = 61;
}





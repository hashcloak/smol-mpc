use crate::rng::Rng;
use crate::mersenne::MersenneField;

/// Implementation of a finite field of prime order.
struct Fp<T: MersenneField> {
    value: T,
}

impl<T> Fp<T> 
where T: MersenneField
{
    fn new(value: T) -> Self {
        todo!()
    }

    /// Generates a random element in the field.
    fn random(rng: Rng) -> Self {
        todo!()
    }
    
    /// Add a two elements in the field
    fn add(&self, other: Self) -> Self {
        todo!()
    }

    /// Negate an element in the field
    fn negate(&self) -> Self {
        todo!()
    }

    /// Multiply two elements in the field
    fn multiply(&self, other: Self) -> Self {
        todo!()
    }
}

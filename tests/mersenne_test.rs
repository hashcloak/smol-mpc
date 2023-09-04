use rand::Rng;
use smol_mpc::math::mersenne::{Mersenne61, MersenneField};
use smol_mpc::utils::prg::Prg;

#[test]
fn mersenne61_new() {
    let new_elem = Mersenne61::new(12);
    assert_eq!(new_elem.value, 12)
}

#[test]
fn mersenne61_new_wraparound() {
    let new_elem = Mersenne61::new(Mersenne61::ORDER + 1);
    assert_eq!(new_elem.value, 1);
}

#[test]
fn mersenne61_add() {
    let a = Mersenne61::new(2);
    let b = Mersenne61::new(3);

    let sum = a.add(&b);
    assert_eq!(sum.value, 5);
}

#[test]
fn mersenne61_add_wraparound() {
    let a = Mersenne61::new(Mersenne61::ORDER - 2);
    let b = Mersenne61::new(5);

    let sum = a.add(&b);
    assert_eq!(sum.value, 3)
}

#[test]
fn mersenne61_mult() {
    let a = Mersenne61::new(10);
    let b = Mersenne61::new(11);

    let mult = a.multiply(&b);
    assert_eq!(mult.value, 110);
}

#[test]
fn mersenne61_mult_wraparound() {
    let a = Mersenne61::new(Mersenne61::ORDER - 1);
    let b: Mersenne61 = Mersenne61::new(2);

    let mult = a.multiply(&b);
    let result = Mersenne61::new(Mersenne61::ORDER - 2);

    assert_eq!(mult.value, result.value);
}

#[test]
fn mersenne61_inverse() {
    let a = Mersenne61::new(10);
    let inv_a = a.inverse();

    let mult = a.multiply(&inv_a);
    assert_eq!(mult.value, 1);
}

#[test]
fn mersenne61_inverse_random() {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..Mersenne61::ORDER);

    let a = Mersenne61::new(num);
    let inv_a = a.inverse();

    let mult = a.multiply(&inv_a);
    assert_eq!(mult.value, 1);
}

#[test]
fn mersenne61_prg() {
    let mut prg = Prg::new(Some(vec![0x4a, 0x4b]));
    let rand_mersenne = Mersenne61::random(&mut prg);

    let product = rand_mersenne.multiply(&rand_mersenne.inverse());
    assert_eq!(product.value, 1);
}

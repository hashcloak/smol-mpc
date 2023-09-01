use std::vec;

use smol::math::mersenne::{Mersenne61, MersenneField};
use smol::mpc;
use smol::utils::prg::Prg;
use smol::vm::VirtualMachine;

type Fp = Mersenne61;

#[test]
fn mpc_distribute_share() {
    let mut prg = Prg::new(None);

    let mut alice: VirtualMachine<Fp> = VirtualMachine::new("alice");
    let mut bob: VirtualMachine<Fp> = VirtualMachine::new("bob");

    alice.insert_priv_value("a", Fp::new(4));

    mpc::distribute_shares("a", "alice", vec![&mut alice, &mut bob], &mut prg);

    let share_alice = alice.get_share("a");
    let share_bob = bob.get_share("a");

    let reconstruction = share_alice.value.add(&share_bob.value);

    assert_eq!(reconstruction.value, 4);
}

#[test]
fn reconstruct_share() {
    let mut prg = Prg::new(None);

    let mut alice: VirtualMachine<Fp> = VirtualMachine::new("alice");
    let mut bob: VirtualMachine<Fp> = VirtualMachine::new("bob");

    alice.insert_priv_value("a", Fp::new(4));
    mpc::distribute_shares("a", "alice", vec![&mut alice, &mut bob], &mut prg);

    let reconstructed_value = mpc::reconstruct_share(&mut vec![&mut alice, &mut bob], "a");
    assert_eq!(reconstructed_value.value, 4);
}

#[test]
fn add_protocol() {
    let mut prg = Prg::new(None);

    let mut alice: VirtualMachine<Fp> = VirtualMachine::new("alice");
    let mut bob: VirtualMachine<Fp> = VirtualMachine::new("bob");

    alice.insert_priv_value("a", Fp::new(4));
    mpc::distribute_shares("a", "alice", vec![&mut alice, &mut bob], &mut prg);

    bob.insert_priv_value("b", Fp::new(2));
    mpc::distribute_shares("b", "bob", vec![&mut alice, &mut bob], &mut prg);

    mpc::add_protocol(&mut vec![&mut alice, &mut bob], "a", "b", "c");

    let sum = mpc::reconstruct_share(&mut vec![&mut alice, &mut bob], "c");
    assert_eq!(sum.value, 6);
}

#[test]
fn simulate_random_distribution() {
    let mut prg = Prg::new(None);

    let mut alice: VirtualMachine<Fp> = VirtualMachine::new("alice");
    let mut bob: VirtualMachine<Fp> = VirtualMachine::new("bob");

    let value = Fp::new(10);
    mpc::simulate_random_dist("a", &mut vec![&mut alice, &mut bob], &value, &mut prg);

    let reconstruction = mpc::reconstruct_share(&mut vec![&mut alice, &mut bob], "a");
    assert_eq!(reconstruction.get_value(), 10);
}

#[test]
fn generate_triple() {
    let mut prg = Prg::new(None);

    let mut alice: VirtualMachine<Fp> = VirtualMachine::new("alice");
    let mut bob: VirtualMachine<Fp> = VirtualMachine::new("bob");

    mpc::generate_triple(&mut vec![&mut alice, &mut bob], ("a", "b", "c"), &mut prg);
    let rec_a = mpc::reconstruct_share(&mut vec![&mut alice, &mut bob], "a");
    let rec_b = mpc::reconstruct_share(&mut vec![&mut alice, &mut bob], "b");
    let rec_c = mpc::reconstruct_share(&mut vec![&mut alice, &mut bob], "c");

    assert_eq!(rec_a.multiply(&rec_b).get_value(), rec_c.get_value());
}

#[test]
fn mult_by_const() {
    let mut prg = Prg::new(None);

    let mut alice: VirtualMachine<Fp> = VirtualMachine::new("alice");
    let mut bob: VirtualMachine<Fp> = VirtualMachine::new("bob");

    alice.insert_priv_value("a", Fp::new(4));
    mpc::distribute_shares("a", "alice", vec![&mut alice, &mut bob], &mut prg);

    let pub_val = Fp::new(6);
    mpc::multiply_by_const_protocol(&mut vec![&mut alice, &mut bob], &pub_val, "a", "m");
    let reconst = mpc::reconstruct_share(&mut vec![&mut alice, &mut bob], "m");
    assert_eq!(reconst.value, 24);
}

#[test]
fn multiplication() {
    let mut prg = Prg::new(Some(vec![1, 2]));

    let mut alice: VirtualMachine<Fp> = VirtualMachine::new("alice");
    let mut bob: VirtualMachine<Fp> = VirtualMachine::new("bob");

    mpc::generate_triple(
        &mut vec![&mut alice, &mut bob],
        ("x1", "x2", "x3"),
        &mut prg,
    );

    let x1 = mpc::reconstruct_share(&mut vec![&mut alice, &mut bob], "x1");
    let x2 = mpc::reconstruct_share(&mut vec![&mut alice, &mut bob], "x2");
    let x3 = mpc::reconstruct_share(&mut vec![&mut alice, &mut bob], "x3");
    println!("X1 value: {:?}", x1.value);
    println!("X2 value: {:?}", x2.value);
    println!("X3 value: {:?}", x3.value);

    alice.insert_priv_value("a", Fp::new(4));
    mpc::distribute_shares("a", "alice", vec![&mut alice, &mut bob], &mut prg);

    bob.insert_priv_value("b", Fp::new(2));
    mpc::distribute_shares("b", "bob", vec![&mut alice, &mut bob], &mut prg);

    mpc::mult_protocol(
        &mut vec![&mut alice, &mut bob],
        "a",
        "b",
        "prod",
        ("x1", "x2", "x3"),
    );

    let mult_reconst = mpc::reconstruct_share(&mut vec![&mut alice, &mut bob], "prod");

    assert_eq!(mult_reconst.get_value(), 8)
}

#[test]
fn subtract_protocol() {
    let mut prg = Prg::new(None);

    let mut alice: VirtualMachine<Fp> = VirtualMachine::new("alice");
    let mut bob: VirtualMachine<Fp> = VirtualMachine::new("bob");

    alice.insert_priv_value("a", Fp::new(4));
    mpc::distribute_shares("a", "alice", vec![&mut alice, &mut bob], &mut prg);

    bob.insert_priv_value("b", Fp::new(6));
    mpc::distribute_shares("b", "bob", vec![&mut alice, &mut bob], &mut prg);

    mpc::subtract_protocol(&mut vec![&mut alice, &mut bob], "a", "b", "c");

    let subs = mpc::reconstruct_share(&mut vec![&mut alice, &mut bob], "c");
    assert_eq!(subs.value, Fp::ORDER - 2);
}

#[test]
fn distribute_pub_value() {
    let mut alice: VirtualMachine<Fp> = VirtualMachine::new("alice");
    let mut bob: VirtualMachine<Fp> = VirtualMachine::new("bob");

    let value = Fp::new(100);
    mpc::distribute_pub_value(&value, "v", &mut vec![&mut alice, &mut bob]);

    let rec_value = mpc::reconstruct_share(&mut vec![&mut alice, &mut bob], "v");
    assert_eq!(rec_value.get_value(), 100);
}
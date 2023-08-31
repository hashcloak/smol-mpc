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

    let reconstructed_value = mpc::reconstruct_share(vec![&alice, &bob], "a");
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

    mpc::add_protocol(vec![&mut alice, &mut bob], "a", "b", "c");

    let sum = mpc::reconstruct_share(vec![&alice, &bob], "c");
    assert_eq!(sum.value, 6);
}

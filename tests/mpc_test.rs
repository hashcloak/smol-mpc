use smol::utils::prg::Prg;
use smol::vm::VirtualMachine;
use smol::mpc;
use smol::math::mersenne::{Mersenne61, MersenneField};

type Fp = Mersenne61;

#[test]
fn mpc_distribute_share() {
    let mut prg = Prg::new(None);

    let mut alice: VirtualMachine<Fp> = VirtualMachine::new("alice");
    let mut bob: VirtualMachine<Fp> = VirtualMachine::new("bob");

    alice.insert_priv_value("a", Fp::new(4));

    let parties = vec![&mut alice, &mut bob];
    mpc::distribute_shares("a", "alice", parties, &mut prg);

    let share_alice = alice.get_share("a");
    let share_bob = bob.get_share("b");

    let reconstruction = share_alice.value.add(&share_bob.value);
    
    assert_eq!(reconstruction.value, 4);
}

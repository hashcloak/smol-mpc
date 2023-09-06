//! Smol is a tiny library for learning Secure Multi-party Computation (MPC) using
//! Rust. This library can be considered as a port of [TinySMPC] with some
//! elements taken from [SCL]. The purpose of this library **is not** to implement
//! all the networking and communication required for a real-world implementation 
//! of an MPC protocol. Instead, we aim for a simpler representation in which
//! all of these tasks are done locally and the communication is simulated by
//! sending information between virtual machines that represent nodes in the 
//! network.
//! 
//! The idea of the library is to represent each node in the network as a
//! as a small virtual machine (see [`VirtualMachine`](crate::vm::VirtualMachine)). 
//! Each virtual machine will have a memory of two types. The private memory 
//! will store all the that are known by the virtual machine. The shared memory 
//! will store the shared distributed in a protocol execution. Both types of 
//! memory is implemented in an ID-value structure, which means that all the 
//! values will be retrieved and stored by a user-defined ID of type [`&str`]. 
//! So sending a value from one machine to the other corresponds to retrieving a 
//! value from the first party using the ID, and storing it in the memory of 
//! the other party using the same ID.
//! 
//! At the time of writing, we have implemented a passive protocol based on additive
//! secret-sharing that performs multiplications using beaver triples. Such 
//! Beaver triples **are not** generated using a protocol, instead, its 
//! generation is simulated. In future work, we will work on implementing a 
//! protocol to illustrate the generation of Beaver triples.
//!   
//! # Examples
//! 
//! Here, we present some examples on how to use our library for computing very
//! basic protocols.
//! 
//! ## Secure addition
//! 
//! Let us start by showing an example of how to execute a protocol for secure
//! addition between two parties. In this protocol, two parties, Alice and Bob,
//! want to add their private values.
//! 
//! ```rust
//! use smol_mpc::math::mersenne::{Mersenne61, MersenneField};
//! use smol_mpc::mpc;
//! use smol_mpc::utils::prg::Prg;
//! use smol_mpc::vm::VirtualMachine;
//! 
//! type Fp = Mersenne61;
//! 
//! fn main() {
//!     // Creates a new pseudo-random generator with a default seed.
//!     let mut prg = Prg::new(None);
//!
//!     // Creates two virtual machines that represent the nodes involved in the
//!     // computation. The ID for each virtual machine is provided in the
//!     // constructor to identify each virtual machine during the protocol
//!     // executions.
//!     let mut alice: VirtualMachine<Fp> = VirtualMachine::new("alice");
//!     let mut bob: VirtualMachine<Fp> = VirtualMachine::new("bob");
//!    
//!     // Alice stores in her private memory a value with ID "a". This value is
//!     // known only to alice and no other parties.
//!     alice.insert_priv_value("a", Fp::new(4));
//! 
//!     // Alice distribute shares of her private valued previously stored with
//!     // ID "a" among the vector of parties provided. In this case, the vector
//!     // contains the parties Alice and Bob. At the end of the execution, both
//!     // of them will have a share of the value 4 stored in their share memory
//!     // and identified with ID "a".
//!     mpc::distribute_shares("a", "alice", vec![&mut alice, &mut bob], &mut prg);
//!
//!     // Bob stores in his private memory a value with ID "b".
//!     bob.insert_priv_value("b", Fp::new(2));
//! 
//!     // Bob distributes shares of its private value "b" among him and Alice.
//!     // At the end, both will have a share of the value 2 in their share
//!     // memory stored with id "b".
//!     mpc::distribute_shares("b", "bob", vec![&mut alice, &mut bob], &mut prg);
//!
//!     // Alice and Bob engage in an addition protocol to securely add "a" and
//!     // "b". The result of this protocol will be shares of the sum of both
//!     // private values. Such share will be stored in the share memory of both
//!     // parties using the id "c" provided as the last parameter.
//!     mpc::add_protocol(&mut vec![&mut alice, &mut bob], "a", "b", "c");
//!
//!     // Once the sum protocol is completed, Alice and Bob engage in a
//!     // protocol to reconstruct a secret-shared value. In this case, they
//!     // want to reconstruct the sum of "a" and "b", whose shares have been
//!     // computed in the previous step and stored under the ID "c". So they
//!     // recomstruct the value of "c".
//!     let sum = mpc::reconstruct_share(&mut vec![&mut alice, &mut bob], "c");
//! }
//! ```
//! 
//! ## Secure multiplication
//! 
//! Now, let us show a an example of how to perform a multiplication using the
//! library. In this example, two parties, Alice and Bob, want to perfrom a 
//! secure multiplication providing their private values.
//! 
//! ```rust
//! use smol_mpc::math::mersenne::{Mersenne61, MersenneField};
//! use smol_mpc::mpc;
//! use smol_mpc::utils::prg::Prg;
//! use smol_mpc::vm::VirtualMachine;
//! 
//! type Fp = Mersenne61;
//! 
//! fn main () {
//!     // Creates a new pseudo-random generator.
//!     let mut prg = Prg::new(Some(vec![1, 2]));
//! 
//!     // These two lines creates two virtual machines, one for Alice and the
//!     // other for Bob.
//!     let mut alice: VirtualMachine<Fp> = VirtualMachine::new("alice");
//!     let mut bob: VirtualMachine<Fp> = VirtualMachine::new("bob");
//!     
//!     // Alice distributes a private value. Here, Alice and Bob obtain shares
//!     // of a value stored with ID "a".
//!     alice.insert_priv_value("a", Fp::new(4));
//!     mpc::distribute_shares("a", "alice", vec![&mut alice, &mut bob], &mut prg);
//! 
//!     // Bob distributes a private value. Here, Alice and Bob obtain shares
//!     // of a value stored with ID "b"
//!     bob.insert_priv_value("b", Fp::new(2));
//!     mpc::distribute_shares("b", "bob", vec![&mut alice, &mut bob], &mut prg);
//! 
//!     // Here, Alice and Bob receive shares of a Beaver triple (x1, x2, x3),
//!     // where x3 = x1 * x2. Such shares are stored in the memory of alice and
//!     // Bob with IDs "x1", "x2" and "x3" respectively.
//!     mpc::generate_triple(
//!         &mut vec![&mut alice, &mut bob],
//!         ("x1", "x2", "x3"),
//!         &mut prg,
//!     );
//! 
//!     // Alice and Bob engage in a multiplication protocol to compute securely
//!     // the product of "a" with "b", using the triple whose ID's are "x1",
//!     // "x2" and "x3" (the same ones that were created in the previous)
//!     // instruction. At the end of the computation, Alice and Bob will obtain
//!     // shares of the product of "a" and "b", and such share will be stored
//!     // in the memory using the id "prod".
//!     mpc::mult_protocol(
//!         &mut vec![&mut alice, &mut bob],
//!         "a",
//!         "b",
//!         "prod",
//!         ("x1", "x2", "x3"),
//!     );
//!    
//!     // Alice and Bob engage in a protocol to reconstruct the value of "prod".
//!     let mult_reconst = mpc::reconstruct_share(&mut vec![&mut alice, &mut bob], "prod");
//! }
//! ```
//! 
//! # Disclaimer
//! 
//! We stress that the work presented here is purely educational and does not 
//! intend to show a secure or efficient implementation. The core of the library
//! is to give the user an idea of how protocols work at a very high level. So, 
//! the implementation may have security issues and sometimes it may not 
//! represent all the details and caveats of a real-world secure and efficient 
//! implementation of the techniques covered here.
//! 
//! [TinySMPC]: https://github.com/kennysong/tinysmpc
//! [SCL]: https://github.com/anderspkd/secure-computation-library

pub mod math;
pub mod mpc;
pub mod utils;
pub mod vm;

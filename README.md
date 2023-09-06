# smol-mpc

![Rust](https://github.com/hashcloak/smol-mpc/actions/workflows/rust.yml/badge.svg)
[![Dependency status](https://deps.rs/repo/github/hashcloak/smol-mpc/status.svg)](https://deps.rs/repo/github/hashcloak/smol-mpc)
[![Documentation](https://docs.rs/smol-mpc/badge.svg)](https://docs.rs/smol-mpc/latest/smol_mpc/)

Smol-mpc is a tiny library to learn Secure Multiparty Computation (MPC) basics using
the Rust programming language. Smol-mpc allows to the user to experiment with toy
examples of MPC protocols and allows him/her to understand the basic concepts
behind MPC. We developed Smol-mpc based on the ideas present in [TinySMPC](https://github.com/kennysong/tinysmpc)
with some elements taken from [Secure Computation Library](https://github.com/anderspkd/secure-computation-library).
In the case of TinySMPC, we took the basic architecture and adapted it to be more
compatible with Rust. Some elements like virtual machines and shares are very
similar, but Smol-mpc differs significantly at some points to give the user a
better experience using Rust. The main difference with Tiny SMPC is the way in 
which we consider the virtual machine. In our case is more similar to the way in 
which ideal functionalities are specified in the theory of MPC, that is, the 
memory of each virtual machine is addressed using IDs for each variable stored.
For Secure Computation Library, we took the ideas used there in the math library
 to implement the $\mathbb{F}_p$ with $p = 2^{61} - 1$ (which is a Mersenne prime)
 as our underlying algebraic structure. We also took the idea from Secure 
Computation Library to implement a pseudo-random generator using AES-CTR.

The documentation for this library and some examples of how to use it can be found [here](https://docs.rs/smol-mpc/).

## Milestones

- [X] ~Fully document the source code.~
- [ ] Add a user-friendly output functionality for the virtual machine states.
- [ ] Write a protocol for Beaver triple generation.
- [ ] Allow support for other protocols.

## Disclaimer

We stress that the work presented here is purely educational and does not intend to show a secure or efficient implementation. The core of the library is to give the user an idea of how protocols work at a very high level. So, the implementation may have security issues and sometimes it may not represent all the details and caveats of a real-world secure and efficient implementation of the techniques covered here.

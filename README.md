# smol

Smol is a tiny library to learn Secure Multiparty Computation (MPC) basics using
the Rust programming language. Smol allows to the user to experiment with toy
examples of MPC protocols and allows him/her to understand the basic concepts
behind MPC. We developed Smol based on the ideas present in [TinySMPC](https://github.com/kennysong/tinysmpc)
with some elements taken from [Secure Computation Library](https://github.com/anderspkd/secure-computation-library).
In the case of TinySMPC, we took the basic architecture and adapted it to be more
compatible with Rust. Some elements like virtual machines and shares are very
similar, but Smol differs significantly at some points to give the user a
better experience using Rust. The main difference with Tiny SMPC is the way in 
which we consider the virtual machine. In our case is more similar to the way in 
which ideal functionalities are specified in the theory of MPC, that is, the 
memory of each virtual machine is addressed using IDs for each variable stored.
For Secure Computation Library, we took the ideas used there in the math library
 to implement a Mersenne61 field as the underlying algebraic structure of our 
protocols.
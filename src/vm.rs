//! Implements a basic representation of a virtual machine.
//!
//! In the context of this library, a virtual machine represents a participant
//! or player in an MPC protocol. In a real-world execution, a participant is
//! a node in a network that receives, processes, and send information according
//! to a protocol specification.

use crate::math::mersenne::MersenneField;
use crate::mpc::Share;
use std::collections::HashMap;

/// Defines a virtual machine.
///
/// The virtual machine is represented as a node that has an ID based memory in
/// which it saves the information. All the machines have an ID used to identify
/// them during a protocol execution. The inspiration of this design comes from
/// the way in wich an ideal functionality is specified in a Universal
/// Composability proof, that is, as a entity that has a ID based memory to
/// store elements and that also sends, process and receives information.
/// However, we stress that this implementation is not the implementation of an
/// ideal functionality, we just take the some elements.
///
/// The memory is divided into two types. The private memory will hold values
/// that a certain node knows but are not secret-shared among the parties. The
/// shares memory stores the shares of a certain value. To make things simple,
/// when a value is public, it is stored in the private memory because, at the
/// end, it is a value that is known all the machines. Each variable stored in
/// the memory has also an ID to refer to it during the protocol execution. In
/// particular, if a value is secret-shared among a certain set of parties, it
/// will have the same ID in memory for all the virtual machines involved in the
/// protocol.
pub struct VirtualMachine<'a, T: MersenneField> {
    /// ID of the virtual machine.
    pub id: &'a str,

    /// Memory for private values.
    pub private_values: HashMap<&'a str, T>,

    /// Memory for shared values.
    pub shares: HashMap<&'a str, Share<'a, T>>,
}

impl<'a, 'b, T: MersenneField> VirtualMachine<'a, T>
where
    'a: 'b,
{
    /// Creates a new virtual machine using a provided ID.
    pub fn new(id_machine: &'a str) -> Self {
        Self {
            id: id_machine,
            private_values: HashMap::new(),
            shares: HashMap::new(),
        }
    }

    /// Inserts a value in the private memory using a provided ID.
    pub fn insert_priv_value(&mut self, id: &'a str, value: T) {
        if self.shares.contains_key(id) {
            panic!("There exists a share with this id");
        }

        self.private_values.insert(id, value);
    }

    /// Insert a share in the share memory using a provided ID.
    pub fn insert_share(&mut self, id: &'a str, share: Share<'a, T>) {
        if self.shares.contains_key(id) {
            panic!("There exists a share with this id.");
        }

        self.shares.insert(id, share);
    }

    /// Returns a private value with the provided id stored in the private
    /// memory.
    pub fn get_priv_value(&'a self, id: &'a str) -> &'b T {
        if let Some(share) = self.private_values.get(id) {
            share
        } else {
            panic!("The id is not registered in the virtual machine.")
        }
    }

    /// Returns the share with the provided ID previously stored in the share
    /// memory.
    pub fn get_share(&'a self, id: &'a str) -> &'b Share<'a, T> {
        if let Some(share) = self.shares.get(id) {
            share
        } else {
            panic!("The id `{}` is not registered in the virtual machine.", id);
        }
    }
}

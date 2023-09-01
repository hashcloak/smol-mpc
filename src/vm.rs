use crate::math::mersenne::MersenneField;
use crate::mpc::{MultTriple, Share};
use std::collections::HashMap;

pub struct VirtualMachine<'a, T: MersenneField> {
    pub id: &'a str,
    pub private_values: HashMap<&'a str, T>,
    pub shares: HashMap<&'a str, Share<'a, T>>,
    pub triples: HashMap<&'a str, MultTriple<'a, T>>,
}

impl<'a, 'b, T: MersenneField> VirtualMachine<'a, T>
where
    'a: 'b,
{
    pub fn new(id_machine: &'a str) -> Self {
        Self {
            id: id_machine,
            private_values: HashMap::new(),
            shares: HashMap::new(),
            triples: HashMap::new(),
        }
    }

    pub fn insert_priv_value(&mut self, id: &'a str, value: T) {
        if self.shares.contains_key(id) {
            panic!("There exists a share with this id");
        }

        self.private_values.insert(id, value);
    }

    pub fn insert_share(&mut self, id: &'a str, share: Share<'a, T>) {
        if self.shares.contains_key(id) {
            panic!("There exists a share with this id.");
        }

        self.shares.insert(id, share);
    }

    pub fn get_priv_value(&'a self, id: &'a str) -> &'b T {
        if let Some(share) = self.private_values.get(id) {
            share
        } else {
            panic!("The id is not registered in the virtual machine.")
        }
    }

    pub fn get_share(&'a self, id: &'a str) -> &'b Share<'a, T> {
        if let Some(share) = self.shares.get(id) {
            share
        } else {
            panic!("The id `{}` is not registered in the virtual machine.", id);
        }
    }

    pub fn get_triple(&'a self, id_triple: &'a str) -> &'b MultTriple<'a, T> {
        if let Some(triple) = self.triples.get(id_triple) {
            triple
        } else {
            panic!(
                "The id `{}` is not registered among the triples of the virtual machine",
                id_triple
            );
        }
    }

    pub fn insert_triple(&'a mut self, id_triple: &'a str, triple: MultTriple<'a, T>) {
        if self.triples.contains_key(id_triple) {
            panic!(
                "The id `{}` is already registered among the triples",
                id_triple
            );
        }
        self.triples.insert(id_triple, triple);
    }
}

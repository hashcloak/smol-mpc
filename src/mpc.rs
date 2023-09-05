//! Implements all the MPC protocols and functionalities to simulate a secure
//! computation.
//! 
//! In an MPC protocol based on secret-sharing, it is common to define five 
//! standar protocols to allow general computation:
//! - A protocol for secure multiplication.
//! - A protocol to perform linear combinations between private values.
//! - A protocol to distribute shares of a value.
//! - A protocol to reconstruct a value from its shares.
//! - A protocol to generate correlated randomness needed in the execution of
//! the protocol.
//! 
//! At the time of writting, we only support one protocol based on additive
//! secret-sharing schemes using Beaver triples for multiplications with passive
//! security. From the list presented above, we only cover the first four 
//! elements. The generation or correlated randomness via secure protocols is 
//! not implemented yet. Those functionalities are emulated using PRGs.

use crate::math::mersenne::MersenneField;
use crate::utils::prg::Prg;
use crate::vm::VirtualMachine;

/// Represents an additive share of a private element in certain algebraic
/// structure. 
/// 
/// We need to make clear that the `value` field **do not** represent the value
/// that the share is trying to hide. On the contrary, this field stores the
/// value that a party holds once the shares of a private element have been
/// computed and distributed.
pub struct Share<'a, T: MersenneField> {
    /// ID of the share in memory.
    pub id: &'a str,

    /// Value that the share holds.
    pub value: T,
}

impl<'a, T: MersenneField> Share<'a, T> {
    /// Creates a new share with a given value.
    fn new(id: &'a str, value: T) -> Self {
        Self { id, value }
    }
}

/// Distributes a share among a set of parties.
/// 
/// This function distributes shares of a value stored in the private memory of 
/// a party with ID `id_owner`. The distribution of shares is done among the
/// parties provided in the parameter `parties`. The shares computed and
/// distributed will be stored in the share memory of each parties with the ID
/// `id_var` (i.e. with the same ID that the owner has in its private memory).
pub fn distribute_shares<'a, 'b, T>(
    id_var: &'a str,
    id_owner: &'a str,
    parties: Vec<&'b mut VirtualMachine<'a, T>>,
    prg: &mut Prg,
) where
    T: MersenneField,
    'a: 'b,
{
    let mut shares: Vec<Share<T>> = Vec::new();
    let mut sum = T::new(0);
    for _ in 0..parties.len() - 1 {
        let random_elem = T::random(prg);
        sum = sum.add(&random_elem);
        let share_random = Share::new(id_var, random_elem);
        shares.push(share_random);
    }

    let mut value_search = None;
    for party in &parties {
        if party.id == id_owner {
            value_search = Some(party.get_priv_value(id_var));
        }
    }

    let value = value_search.unwrap_or_else(|| {
        panic!("Party with that id does not exist.");
    });

    let last_value = value.subtract(&sum);
    let share_last_value = Share::new(id_var, last_value);
    shares.push(share_last_value);

    for party in parties {
        party.insert_share(id_var, shares.remove(0));
    }
}

/// Multiplicates two secret-shared values distributed among a set of parties.
/// 
/// This protocol executes the multiplication between two secret-shared values
/// whose shares has been distributed and stored in the memory of the parties
/// involved in the protocol. The multiplication is executed using a
/// multiplication triple whose shares have been distributed among the parties.
/// At the end of the execution of the protocol, the parties will end up with
/// the shares of the product under the ID `id_result` stored in the share
/// memory.
pub fn mult_protocol<'a, 'b, T>(
    parties: &mut Vec<&'b mut VirtualMachine<'a, T>>,
    id_x: &'a str,
    id_y: &'a str,
    id_result: &'a str,
    triple_id: (&'a str, &'a str, &'a str),
) where
    T: MersenneField,
    'a: 'b,
{
    // Computing epsilon and delta
    subtract_protocol(&mut *parties, id_x, triple_id.0, "epsilon");
    subtract_protocol(&mut *parties, id_y, triple_id.1, "delta");

    let epsilon = reconstruct_share(&*parties, "epsilon");
    let delta = reconstruct_share(&*parties, "delta");

    multiply_by_const_protocol(&mut *parties, &epsilon, triple_id.1, "t1");
    multiply_by_const_protocol(&mut *parties, &delta, triple_id.0, "t2");

    add_protocol(&mut *parties, "t1", "t2", "sum");
    add_protocol(&mut *parties, "sum", triple_id.2, "sumc");

    distribute_pub_value(&epsilon.multiply(&delta), "epsdelt", &mut *parties);
    add_protocol(&mut *parties, "sumc", "epsdelt", id_result);

    // Free memory of intermediate steps to make variables available.
    for party in parties {
        party.shares.remove("epsilon");
        party.shares.remove("delta");
        party.shares.remove("t1");
        party.shares.remove("t2");
        party.shares.remove("sum");
        party.shares.remove("sumc");
        party.shares.remove("epsdelt");
    }
}

/// Distributes shares of a publicly known value.
/// 
/// This method distributes shares among a set of parties of a publicly known
/// value. The shares are stored in the share memory of each party using the
/// provided ID.
pub fn distribute_pub_value<'a, 'b, T>(
    value: &T,
    id: &'a str,
    parties: &mut [&'b mut VirtualMachine<'a, T>],
) where
    T: MersenneField,
    'a: 'b,
{
    parties[0].insert_share(id, Share::new(id, T::new(value.value())));
    for party in parties.iter_mut().skip(1) {
        party.insert_share(id, Share::new(id, T::new(0)));
    }
}

/// Computes the secure multiplication of a publicly known value with a
/// previously secret-shared value.
/// 
/// The function multiplies a public value with the shares with a private value
/// that has been already secret-shared among the provided set of parties. The
/// result of this computation will be shares of the result stored in the 
/// memory of each party under ID `id_result`.
pub fn multiply_by_const_protocol<'a, 'b, T>(
    parties: &mut Vec<&'b mut VirtualMachine<'a, T>>,
    value: &T,
    id: &'a str,
    id_result: &'a str,
) where
    T: MersenneField,
    'a: 'b,
{
    for party in parties {
        let share = party.get_share(id);
        let value_mult = share.value.multiply(value);

        let share_mult = Share::new(id_result, value_mult);
        party.insert_share(id_result, share_mult);
    }
}

/// Computes the secure subtraction between two secret shared values.
/// 
/// Computes the secure subraction between the shared value stored with ID
/// `id_a` with the value store with ID `id_b`. The result of this function will
/// be be the result of the operation stored as shares under the ID `id_result`.
pub fn subtract_protocol<'a, T>(
    parties: &mut Vec<&mut VirtualMachine<'a, T>>,
    id_a: &'a str,
    id_b: &'a str,
    id_result: &'a str,
) where
    T: MersenneField,
{
    multiply_by_const_protocol(&mut *parties, &T::new(1).negate(), id_b, "subtraction");
    add_protocol(&mut *parties, id_a, "subtraction", id_result);

    // Remove intermediate values
    for party in parties {
        party.shares.remove("subtraction");
    }
}

/// Adds two secret-shared values distributed among a set of parties.
/// 
/// This protocol executes the addition between two secret-shared values
/// whose shares has been distributed and stored in the memory of the parties
/// involved in the protocol. The addition is executed locally by the parties.
/// At the end of the execution of the protocol, the parties will end up with
/// the shares of the addition under the ID `id_result` stored in the share
/// memory.
pub fn add_protocol<'a, T>(
    parties: &mut Vec<&mut VirtualMachine<'a, T>>,
    id_a: &'a str,
    id_b: &'a str,
    id_result: &'a str,
) where
    T: MersenneField,
{
    for party in parties {
        let share_a = party.get_share(id_a);
        let share_b = party.get_share(id_b);

        let value_sum = share_a.value.add(&share_b.value);
        let share_sum = Share {
            id: id_result,
            value: value_sum,
        };
        party.insert_share(id_result, share_sum);
    }
}

pub fn reconstruct_share<T>(parties: &Vec<&mut VirtualMachine<T>>, id: &str) -> T
where
    T: MersenneField,
{
    let mut value = T::new(0);
    for party in parties {
        let share_value = &party.get_share(id).value;
        value = value.add(share_value);
    }

    value
}

pub fn generate_triple<'a, 'b, T>(
    parties: &mut Vec<&'b mut VirtualMachine<'a, T>>,
    id_triple: (&'a str, &'a str, &'a str),
    prg: &mut Prg,
) where
    T: MersenneField,
    'a: 'b,
{
    let a = T::random(&mut *prg);
    let b = T::random(&mut *prg);
    let c = a.multiply(&b);

    simulate_random_dist(id_triple.0, &mut *parties, &a, &mut *prg);
    simulate_random_dist(id_triple.1, &mut *parties, &b, &mut *prg);
    simulate_random_dist(id_triple.2, &mut *parties, &c, &mut *prg);
}

pub fn simulate_random_dist<'a, T>(
    id: &'a str,
    parties: &mut Vec<&mut VirtualMachine<'a, T>>,
    value: &T,
    prg: &mut Prg,
) where
    T: MersenneField,
{
    let mut shares: Vec<Share<T>> = Vec::new();
    let mut sum = T::new(0);
    for _ in 0..parties.len() - 1 {
        let random_elem = T::random(prg);
        sum = sum.add(&random_elem);
        let share_random = Share::new(id, random_elem);
        shares.push(share_random);
    }

    let last_value = value.subtract(&sum);
    let share_last_value = Share::new(id, last_value);
    shares.push(share_last_value);

    for party in parties {
        party.insert_share(id, shares.pop().unwrap());
    }
}

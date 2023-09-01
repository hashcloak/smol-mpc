use rand::random;

use crate::math::mersenne::MersenneField;
use crate::utils::prg::Prg;
use crate::vm::VirtualMachine;

#[derive(Clone)]
pub struct Share<'a, T: MersenneField> {
    pub id: &'a str,
    pub value: T,
}

impl<'a, T: MersenneField> Share<'a, T> {
    fn new(id: &'a str, value: T) -> Self {
        Self { id, value }
    }
}

pub struct MultTriple<'a, T: MersenneField> {
    pub id: &'a str,
    pub shares: (Share<'a, T>, Share<'a, T>, Share<'a, T>),
}

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
}

pub fn distribute_pub_value<'a, 'b, T>(
    value: &T,
    id: &'a str,
    parties: &mut Vec<&'b mut VirtualMachine<'a, T>>,
) where
    T: MersenneField,
    'a: 'b,
{
    parties[0].insert_share(id, Share::new(id, T::new(value.get_value())));
    for i in 1..parties.len() {
        parties[i].insert_share(id, Share::new(id, T::new(0)));
    }
}

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
        let value_mult = share.value.multiply(&value);

        let share_mult = Share::new(id_result, value_mult);
        party.insert_share(id_result, share_mult);
    }
}

pub fn subtract_protocol<'a, 'b, T>(
    parties: &mut Vec<&'b mut VirtualMachine<'a, T>>,
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

pub fn add_protocol<'a, 'b, T>(
    parties: &mut Vec<&'b mut VirtualMachine<'a, T>>,
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

pub fn reconstruct_share<'a, 'b, T>(parties: &Vec<&'b mut VirtualMachine<T>>, id: &'a str) -> T
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

pub fn simulate_random_dist<'a, 'b, T>(
    id: &'a str,
    parties: &mut Vec<&'b mut VirtualMachine<'a, T>>,
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

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

pub fn mult_protocol<T>(
    parties: Vec<&mut VirtualMachine<T>>,
    id_a: &str,
    id_b: &str,
    id_result: &str,
    triple_id: &str,
) where
    T: MersenneField,
{
    todo!()
}

pub fn add_protocol<'a, 'b, T>(
    parties: Vec<&'b mut VirtualMachine<'a, T>>,
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

pub fn reconstruct_share<'a, 'b, T>(parties: Vec<&'b VirtualMachine<T>>, id: &'a str) -> T
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

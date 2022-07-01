use std::collections::VecDeque;

fn chain_recur(
    chain: &mut Vec<(u8, u8)>,
    already_used: &mut Vec<(u8, u8)>,
    to_use: &mut VecDeque<(u8, u8)>,
) -> Option<Vec<(u8, u8)>> {
    // Reset the to_use vector
    while let Some(already_used_domino) = already_used.pop() {
        to_use.push_front(already_used_domino);
    }

    // If there is no more domino to used, evaluate the solution
    if to_use.len() == 0 {
        let first_domino = chain.first()?;
        let last_domino = chain.last()?;
        if first_domino.0 == last_domino.1 {
            return Some(chain.clone().into_iter().collect::<Vec<(u8, u8)>>());
        } else {
            return None;
        }
    }

    // Deal with the recursive case
    let last_domino = chain.last().unwrap().clone();
    while let Some(candidat_domino) = to_use.pop_front() {
        let reverse = last_domino.1 == candidat_domino.1;
        let match_domino = last_domino.1 == candidat_domino.0 || last_domino.1 == candidat_domino.1;

        if !match_domino {
            already_used.push(candidat_domino);
            continue;
        }

        if reverse {
            chain.push((candidat_domino.1, candidat_domino.0));
        } else {
            chain.push(candidat_domino);
        }
        match chain_recur(chain, &mut already_used.clone(), &mut to_use.clone()) {
            Some(chain) => {
                return Some(chain);
            }
            None => {
                already_used.push(chain.pop().unwrap());
            }
        }
    }

    None
}

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.len() == 0 {
        return Some(vec![]);
    }

    let mut chain = input
        .to_vec()
        .into_iter()
        .take(1)
        .collect::<Vec<(u8, u8)>>();
    let mut already_used = Vec::<(u8, u8)>::new();
    let mut to_use = input
        .to_vec()
        .into_iter()
        .skip(1)
        .collect::<VecDeque<(u8, u8)>>();

    chain_recur(&mut chain, &mut already_used, &mut to_use)
}

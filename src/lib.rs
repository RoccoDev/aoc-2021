#[allow(unused_macros)]
macro_rules! example {
    ($($values:expr) +) => {
        &stringify!($($values)*).replace(" ", "\n")
    };
}

pub mod days;

#[macro_use]
extern crate aoc_runner_derive;

aoc_lib! { year = 2021 }

/// Attempts to reduce (in place) the possible values in a list of (K, Vec<V>) pairs,
/// where K is a unique identifier and Vec<V> is a list of possible values.
/// If the attempt is successful the vector should only contain *one element* in each pair.
///
/// # Constraints
/// * The list must **not** contain duplicates.
fn reduce_possibilities<K: Clone, V: Copy + PartialEq>(input: &mut Vec<(K, Vec<V>)>) {
    let old_snap = input.clone();
    loop {
        let snapshot = input.clone();
        let mut changed = 0;
        for (i, possibilities) in snapshot.iter().enumerate() {
            match possibilities.1.len() {
                0 => {
                    input[i] = old_snap[i].clone();
                }
                1 => {
                    let x = possibilities.1[0];
                    for (i1, poss2) in input.iter_mut().enumerate() {
                        if i != i1 {
                            let old_len = poss2.1.len();
                            *poss2 = (
                                poss2.0.clone(),
                                poss2
                                    .1
                                    .iter()
                                    .filter_map(|y| if *y != x { Some(*y) } else { None })
                                    .collect::<Vec<_>>(),
                            );
                            if poss2.1.len() != old_len {
                                changed += 1;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        if changed == 0 {
            break;
        }
    }
}

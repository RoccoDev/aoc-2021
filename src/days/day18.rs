use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use itertools::Itertools;

// This needs some serious refactoring

#[derive(Debug)]
enum PairItem {
    Single(i32),
    Pair(Rc<Pair>),
}

#[derive(Debug)]
struct Pair {
    value: RefCell<(PairItem, PairItem)>,
    parent: RefCell<Option<Rc<Pair>>>,
}

impl Default for PairItem {
    fn default() -> Self {
        PairItem::Single(0)
    }
}

impl Pair {
    pub fn reduce(self: &Rc<Self>) {
        loop {
            if self.explode() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }

    pub fn magnitude(&self) -> i32 {
        3 * match &self.value.borrow().0 {
            PairItem::Single(v) => *v,
            PairItem::Pair(p) => p.magnitude(),
        } + 2 * match &self.value.borrow().1 {
            PairItem::Single(v) => *v,
            PairItem::Pair(p) => p.magnitude(),
        }
    }

    pub fn add(self: &Rc<Self>, rhs: &Rc<Self>) -> Rc<Self> {
        let pair = Rc::new(Pair {
            value: RefCell::default(),
            parent: RefCell::default(),
        });
        self.parent.replace(Some(Rc::clone(&pair)));
        rhs.parent.replace(Some(Rc::clone(&pair)));
        pair.value.replace((PairItem::Pair(Rc::clone(self)), PairItem::Pair(Rc::clone(rhs))));
        pair
    }

    fn split(self: &Rc<Self>) -> bool {
        let mapper = |i: &i32| {
            let div = *i as f64 / 2f64;
            Rc::new(Pair {
                value: RefCell::new((PairItem::Single(div.floor() as i32), PairItem::Single(div.ceil() as i32))),
                parent: Some(Rc::clone(self)).into(),
            })
        };

        if let PairItem::Pair(child) = &self.value.borrow().0 {
            if child.split() {
                return true;
            }
        }
        let split = {
            let mut value = self.value.borrow_mut();
            let value = value.deref_mut();

            match value {
                (PairItem::Single(i), _) if *i >= 10 => {
                    value.0 = PairItem::Pair((mapper)(i));
                    true
                }
                (_, PairItem::Single(i)) if *i >= 10 => {
                    value.1 = PairItem::Pair((mapper)(i));
                    true
                }
                _ => false
            }
        };
        if split {
            return true;
        }
        if let PairItem::Pair(child) = &self.value.borrow().1 {
            if child.split() {
                return true;
            }
        }
        false
    }

    fn explode(self: &Rc<Self>) -> bool {
        let mut count = 0;
        self.count_parents(&mut count);
        let child = if let PairItem::Pair(child) = &self.value.borrow().0 {
            Some(Rc::clone(child))
        } else {
            None
        };
        if let Some(c) = child {
            if c.explode() {
                return true;
            }
        }
        let child = if let PairItem::Pair(child) = &self.value.borrow().1 {
            Some(Rc::clone(child))
        } else {
            None
        };
        if let Some(c) = child {
            if c.explode() {
                return true;
            }
        }
        if count < 4 {
            return false;
        }
        let (a, b) = {
            match self.value.borrow().deref() {
                (PairItem::Single(v1), PairItem::Single(v2)) => (*v1, *v2),
                _ => return false
            }
        };
        if self.regular_child(a, true, false) | self.regular_child(b, false, false) {
            let parent = self.parent.borrow();
            let parent = parent.as_ref().unwrap();
            let mut parent = parent.value.borrow_mut();
            let parent = parent.deref_mut();
            let take_first = if let PairItem::Pair(p) = &parent.0 {
                Rc::ptr_eq(p, self)
            } else {
                false
            };
            if take_first {
                std::mem::take(&mut parent.0);
            } else {
                std::mem::take(&mut parent.1);
            }
            true
        } else {
            false
        }
    }

    fn regular_child(self: &Rc<Self>, add: i32, left: bool, go_down: bool) -> bool {
        if go_down {
            // Traverse the tree downwards
            let elem = &mut RefCell::borrow_mut(&self.value);
            let eff_left = !left;
            let elem = if eff_left { &mut elem.0 } else { &mut elem.1 };
            match elem {
                PairItem::Single(v) => {
                    *v += add;
                    true
                }
                PairItem::Pair(p) => {
                    p.regular_child(add, left, true) // we changed directions, we always want the left one (closest)
                }
            }
        } else {
            match &*self.parent.borrow() {
                Some(parent) => {
                    // Get elements to the left (or right, depending on the `left` parameter) of the parent element
                    let next = {
                        let elem = &mut RefCell::borrow_mut(&parent.value);
                        let elem = if left { &mut elem.0 } else { &mut elem.1 };
                        match elem {
                            PairItem::Single(v) => {
                                *v += add;
                                None
                            }
                            PairItem::Pair(p) => {
                                Some(Rc::clone(p))
                            }
                        }
                    };
                    if next.is_none() {
                        return true;
                    }
                    let next = next.as_ref().unwrap();
                    if Rc::ptr_eq(self, next) {
                        parent.regular_child(add, left, false)
                    } else {
                        next.regular_child(add, left, true)
                    }
                }
                None => false
            }
        }
    }

    fn count_parents(&self, count: &mut usize) {
        if let Some(parent) = &*RefCell::borrow(&self.parent) {
            *count += 1;
            parent.count_parents(count)
        }
    }

    pub fn parse(input: &mut &str, parent: RefCell<Option<Rc<Pair>>>) -> Rc<Pair> {
        assert_eq!(input.chars().next().unwrap(), '[');
        let mut level = vec![];
        let pair = Rc::new(Pair {
            value: RefCell::new(Default::default()),
            parent,
        });
        while !input.is_empty() {
            let first = input.chars().next().unwrap();
            *input = &input[1..];
            if first == ']' {
                break;
            }
            let item = Pair::parse_item(input, Some(Rc::clone(&pair)).into());
            level.push(item);
        }
        let items = (std::mem::take(&mut level[0]), std::mem::take(&mut level[1]));
        pair.value.replace(items);
        pair
    }

    fn parse_item(input: &mut &str, parent: RefCell<Option<Rc<Pair>>>) -> PairItem {
        let first = input.chars().next().unwrap();
        match first {
            '[' => {
                PairItem::Pair(Pair::parse(input, parent))
            }
            _ => {
                let num = input.chars().take_while(|c| c.is_ascii_digit()).join("");
                *input = &input[num.len()..];
                PairItem::Single(num.parse().unwrap())
            }
        }
    }
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day18, part1)]
fn part1(input: &[String]) -> i32 {
    let res = input
        .iter()
        .map(|s| Pair::parse(&mut s.as_str(), None.into()))
        .reduce(|acc, n| {
            let tmp = acc.add(&n);
            tmp.reduce();
            tmp
        }).unwrap();
    res.magnitude()
}

#[aoc(day18, part2)]
fn part2(input: &[String]) -> i32 {
    input.iter().permutations(2).map(|s| {
        let res = s
            .iter()
            .map(|s| Pair::parse(&mut s.as_str(), None.into()))
            .reduce(|acc, n| {
                let tmp = acc.add(&n);
                tmp.reduce();
                tmp
            }).unwrap();
        res.magnitude()
    }).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;
        assert_eq!(part1(&parse(input)), 4140);
    }

    #[test]
    fn part2_example() {
        let input = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;
        assert_eq!(part2(&parse(input)), 3993);
    }
}

use std::collections::HashSet;
use std::fs;
use std::ops::RangeInclusive;

fn main() {
    let input = fs::read_to_string("inputs/18.txt").unwrap();

    let set: HashSet<_> = input.lines().map(into_triplet).collect();
    // part A
    let nadjs: usize = set.iter()
        .copied()
        .flat_map(neighbors)
        .filter(|t| !set.contains(t))
        .count();
    println!("{nadjs}");

    // part B

    // i am overengineering
    let range = {
        let (min, max) = set.iter()
            .flat_map(|&(a, b, c)| [a, b, c])
            .fold(None, |minmax, t| {
                minmax
                    .map(|(u1, u2)| (t.min(u1), t.max(u2)))
                    .or(Some((t, t)))
            }).unwrap();
        (min - 1) ..= (max + 1)
    };

    let adjs: usize = outer_scan(&set, range).into_iter()
        .flat_map(neighbors)
        .filter(|t| set.contains(t))
        .count();
    println!("{adjs}");
}

type Triplet = (isize, isize, isize);
fn into_triplet(s: &str) -> Triplet {
    let [a, b, c]: [_; 3] = s.split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    (a, b, c)
}

fn neighbors((a, b, c): Triplet) -> [Triplet; 6] {
    [
        (a + 1, b, c),
        (a - 1, b, c),
        (a, b + 1, c),
        (a, b - 1, c),
        (a, b, c + 1),
        (a, b, c - 1),
    ]
}

// part B
fn outer_scan(set: &HashSet<Triplet>, range: RangeInclusive<isize>) -> HashSet<Triplet> 
{
    let mut outer = HashSet::new();
    let mut scan = vec![(*range.start(), *range.start(), *range.start())];

    while let Some(t) = scan.pop() {
        neighbors(t)
            .into_iter()
            .filter(|n| !set.contains(n))
            .filter(|(a, b, c)| range.contains(a) && range.contains(b) && range.contains(c))
            .for_each(|n| {
                if outer.insert(n) {
                    scan.push(n);
                }
            })
    }

    outer
}
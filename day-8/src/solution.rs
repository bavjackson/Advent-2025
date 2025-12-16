use std::{cmp::Reverse, collections::HashSet};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Coord3D {
    x: u64,
    y: u64,
    z: u64,
}

impl Coord3D {
    fn squared_distance_to(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x)).pow(2)
            + (self.y.abs_diff(other.y)).pow(2)
            + (self.z.abs_diff(other.z)).pow(2)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Dist<'a> {
    c1: &'a Coord3D,
    c2: &'a Coord3D,
    dist: u64,
}

pub fn part1(coords: &Vec<Coord3D>, num_connections: usize) -> String {
    let mut distances: Vec<Dist> = vec![];
    let length = coords.len();

    for i in 0..length {
        for j in (i + 1)..length {
            if i == j {
                continue;
            }

            let c1 = &coords[i];
            let c2 = &coords[j];

            distances.push(Dist {
                c1,
                c2,
                dist: c1.squared_distance_to(c2),
            })
        }
    }

    distances.sort_by_key(|k| k.dist);

    let mut networks: Vec<HashSet<&Coord3D>> = vec![];

    'outer: for dist in distances.iter().take(num_connections) {
        let mut c1_network: Option<(usize, &mut HashSet<&Coord3D>)> = None;
        let mut c2_network: Option<(usize, &mut HashSet<&Coord3D>)> = None;

        for net in networks.iter_mut().enumerate() {
            if net.1.contains(dist.c1) && net.1.contains(dist.c2) {
                continue 'outer;
            } else if net.1.contains(dist.c1) {
                c1_network = Some(net);
            } else if net.1.contains(dist.c2) {
                c2_network = Some(net);
            }
        }

        if c1_network.is_some() && c2_network.is_some() {
            if let (Some((_, c1)), Some((id2, c2))) = (c1_network, c2_network) {
                c1.extend(c2.iter());
                networks.remove(id2);
            }
            continue 'outer;
        } else if c1_network.is_some() && c2_network.is_none() {
            if let Some((_, n)) = c1_network {
                n.insert(dist.c2);
            }
        } else if c1_network.is_none() && c2_network.is_some() {
            if let Some((_, n)) = c2_network {
                n.insert(dist.c1);
            }
        } else if c1_network.is_none() && c2_network.is_none() {
            networks.push(HashSet::from([dist.c1, dist.c2]));
        } else {
            println!("not sure what happened here");
        }
    }

    networks.sort_by_key(|n| Reverse(n.len()));

    networks
        .iter()
        .take(3)
        .fold(1, |acc, n| acc * n.len())
        .to_string()
}

pub fn part2(coords: &Vec<Coord3D>) -> String {
    let mut distances: Vec<Dist> = vec![];
    let length = coords.len();

    for i in 0..length {
        for j in (i + 1)..length {
            if i == j {
                continue;
            }

            let c1 = &coords[i];
            let c2 = &coords[j];

            distances.push(Dist {
                c1,
                c2,
                dist: c1.squared_distance_to(c2),
            })
        }
    }

    distances.sort_by_key(|k| k.dist);

    let mut networks: Vec<HashSet<&Coord3D>> = vec![];

    'outer: for dist in distances.iter() {
        let mut c1_network: Option<(usize, &mut HashSet<&Coord3D>)> = None;
        let mut c2_network: Option<(usize, &mut HashSet<&Coord3D>)> = None;

        for net in networks.iter_mut().enumerate() {
            if net.1.contains(dist.c1) && net.1.contains(dist.c2) {
                continue 'outer;
            } else if net.1.contains(dist.c1) {
                c1_network = Some(net);
            } else if net.1.contains(dist.c2) {
                c2_network = Some(net);
            }
        }

        match (c1_network, c2_network) {
            (None, None) => networks.push(HashSet::from([dist.c1, dist.c2])),
            (Some((_, c1)), Some((id2, c2))) => {
                c1.extend(c2.iter());
                networks.remove(id2);
            }
            (Some((_, c1)), None) => {
                c1.insert(dist.c2);
            }
            (None, Some((_, c2))) => {
                c2.insert(dist.c1);
            }
        }

        if networks[0].len() == coords.len() {
            println!("return here");
            return (dist.c1.x * dist.c2.x).to_string();
        }
    }

    unreachable!();
}

pub fn parse_input(input: &str) -> Vec<Coord3D> {
    let mut output = vec![];
    for line in input.lines() {
        let mut coord = Coord3D { x: 0, y: 0, z: 0 };

        let split = line.split(',');

        for (index, num) in split.into_iter().enumerate() {
            if index == 0 {
                coord.x = num.parse::<u64>().unwrap();
            } else if index == 1 {
                coord.y = num.parse::<u64>().unwrap();
            } else if index == 2 {
                coord.z = num.parse::<u64>().unwrap();
            }
        }

        output.push(coord)
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

    #[test]
    fn part_1() {
        let coords = parse_input(EXAMPLE_INPUT);
        let res = part1(&coords, 10);
        assert_eq!(res, "40");
    }

    #[test]
    fn part_2() {
        let coords = parse_input(EXAMPLE_INPUT);
        let res = part2(&coords);
        assert_eq!(res, "25272");
    }
}

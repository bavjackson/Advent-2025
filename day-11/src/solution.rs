use std::collections::{HashMap, VecDeque};

type DeviceCollection<'a> = HashMap<&'a str, Vec<&'a str>>;

pub fn part1(input: &str) -> String {
    let devices = parse_input(input);

    let mut cache: HashMap<&str, u64> = HashMap::new();

    paths_between(&devices, &mut cache, "you", "out").to_string()
}

pub fn part2(input: &str) -> String {
    let devices = parse_input(input);

    let mut cache_1: HashMap<&str, u64> = HashMap::new();
    let mut cache_2: HashMap<&str, u64> = HashMap::new();
    let mut cache_3: HashMap<&str, u64> = HashMap::new();

    let count = paths_between(&devices, &mut cache_1, "svr", "fft")
        * paths_between(&devices, &mut cache_2, "fft", "dac")
        * paths_between(&devices, &mut cache_3, "dac", "out");

    count.to_string()
}

fn paths_between<'a>(
    devices: &'a DeviceCollection,
    cache: &mut HashMap<&'a str, u64>,
    start: &'a str,
    end: &str,
) -> u64 {
    if cache.contains_key(start) {
        return cache[start];
    }
    if start == end {
        return 1;
    }

    let mut count = 0;
    let initial = devices.get(start);

    match initial {
        Some(keys) => {
            for key in keys.iter() {
                count += paths_between(devices, cache, key, end)
            }

            cache.insert(start, count);
            count
        }
        None => 0,
    }
}

fn parse_input<'a>(input: &'a str) -> DeviceCollection<'a> {
    let mut devices: DeviceCollection = HashMap::new();

    for line in input.lines() {
        let split = line.split_once(": ");

        if let Some((k, v)) = split {
            _ = devices.insert(k, v.split(" ").collect())
        }
    }

    devices
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_PT_1: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

    const EXAMPLE_INPUT_PT_2: &str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;

    #[test]
    fn part_1() {
        let res = part1(EXAMPLE_INPUT_PT_1);
        assert_eq!(res, "5");
    }

    #[test]
    fn part_2() {
        let res = part2(EXAMPLE_INPUT_PT_2);
        assert_eq!(res, "2");
    }
}

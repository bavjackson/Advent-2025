use std::collections::{HashSet, VecDeque};

use good_lp::{
    Expression, IntoAffineExpression, Solution, SolverModel, highs, variable, variables,
};
use regex::Regex;

#[derive(Debug, Clone)]
struct Indicator {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u32>,
}

const LIGHTS_AND_JOLTAGES: &str = r"\[([^\]]+)\].*\{([^}]+)\}";
// const BUTTONS: &str = r"\(([&)]+)\)";
const BUTTONS: &str = r"\(([^)]+)\)";

pub fn part1(input: &str) -> String {
    let mut total = 0;

    let indicators = parse_input(input);

    for indicator in indicators.iter() {
        let mut seen = HashSet::new();
        let mut q = VecDeque::new();

        q.push_back((vec![false; indicator.lights.len()], 0usize));

        while let Some((state, n)) = q.pop_front() {
            if seen.contains(&state) {
                continue;
            }
            if state == indicator.lights {
                total += n;
                break;
            }

            seen.insert(state.clone());

            for button in &indicator.buttons {
                let mut next = state.clone();

                for i in button {
                    next[*i] = !next[*i];
                }
                q.push_back((next, n + 1));
            }
        }
    }

    total.to_string()
}

pub fn part2(input: &str) -> String {
    let mut total: f64 = 0.0;

    let indicators = parse_input(input);

    for indicator in indicators.iter() {
        let mut vars = variables!();

        let presses = (0..indicator.buttons.len())
            .map(|_| vars.add(variable().min(0).integer()))
            .collect::<Vec<_>>();

        let mut problem = highs(vars.minimise(presses.iter().sum::<Expression>()));

        let mut exprs = vec![0.into_expression(); (&indicator.joltages).len()];

        for i in 0..indicator.buttons.len() {
            for &x in &indicator.buttons[i] {
                exprs[x] += presses[i];
            }
        }

        for (e, j) in exprs.into_iter().zip(indicator.joltages.clone()) {
            problem.add_constraint(e.eq(j as f64));
        }

        let sol = problem.solve().unwrap();
        let p = presses.iter().map(|&p| sol.value(p)).collect::<Vec<f64>>();
        // println!("{:?}", p);
        total += p.iter().fold(0.0, |acc, f| acc + f);
    }

    total.to_string()
}

fn parse_input(input: &str) -> Vec<Indicator> {
    let re = Regex::new(LIGHTS_AND_JOLTAGES).unwrap();
    let buttons_re = Regex::new(BUTTONS).unwrap();

    let mut indicators = vec![];

    for line in input.lines() {
        let mut indicator = Indicator {
            lights: vec![],
            buttons: vec![],
            joltages: vec![],
        };

        if let Some(captures) = re.captures(line) {
            indicator.lights = captures[1].chars().map(|ch| ch == '#').collect();
            indicator.joltages = captures[2]
                .split(',')
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
        }

        indicator.buttons = buttons_re
            .captures_iter(line)
            .map(|c| {
                c.get(1)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        // println!("{}", line);
        // println!("{:?}", indicator);

        indicators.push(indicator);
    }

    indicators
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    #[test]
    fn part_1() {
        let res = part1(EXAMPLE_INPUT);
        assert_eq!(res, "7");
    }

    #[test]
    fn part_2() {
        let res = part2(EXAMPLE_INPUT);
        assert_eq!(res, "33");
    }
}

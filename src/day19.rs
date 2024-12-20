use std::collections::{HashMap, HashSet};

#[inline]
fn parse_input<'a>(input: &'a str) -> (HashSet<&'a str>, impl Iterator<Item = &'a str>) {
    let mut lines = input.lines();
    let existing_patterns: HashSet<&'a str> =
        lines.next().expect("Input was empty").split(", ").collect();
    // Consume empty line
    lines.next();
    (existing_patterns, lines)
}

#[inline]
fn is_possible<'a>(
    rem: &'a str,
    patterns: &HashSet<&'a str>,
    index: usize,
    dp: &mut HashMap<&'a str, bool>,
) -> bool {
    if index > rem.len() {
        return false;
    }
    if let Some(res) = dp.get(rem) {
        return *res;
    }
    let res = {
        if patterns.contains(&rem[..index]) {
            if index == rem.len() {
                true
            } else {
                is_possible(&rem[index..], patterns, 1, dp)
                    || is_possible(rem, patterns, index + 1, dp)
            }
        } else {
            is_possible(rem, patterns, index + 1, dp)
        }
    };
    dp.insert(rem, res);
    res
}

#[aoc(day19, part1)]
#[inline]
fn part1(input: &str) -> usize {
    let (patterns, towels) = parse_input(input);
    let mut dp = HashMap::new();
    towels
        .filter(|t| is_possible(t, &patterns, 1, &mut dp))
        .count()
}

#[inline]
fn num_ways<'a>(
    rem: &'a str,
    patterns: &HashSet<&'a str>,
    index: usize,
    dp: &mut HashMap<&'a str, u64>,
) -> u64 {
    if index > rem.len() {
        return 0;
    }
    if let Some(res) = dp.get(rem) {
        return *res;
    }
    let res = {
        if patterns.contains(&rem[..index]) {
            if index == rem.len() {
                return 1;
            } else {
                num_ways(&rem[index..], patterns, 1, dp) + num_ways(rem, patterns, index + 1, dp)
            }
        } else {
            num_ways(rem, patterns, index + 1, dp)
        }
    };
    dp.insert(rem, res);
    res
}

#[aoc(day19, part2)]
#[inline]
fn part2(input: &str) -> u64 {
    let (patterns, towels) = parse_input(input);
    let mut dp = HashMap::new();
    towels.map(|t| num_ways(t, &patterns, 1, &mut dp)).sum()
}

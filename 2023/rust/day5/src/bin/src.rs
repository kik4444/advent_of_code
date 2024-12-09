use std::ops::Range;

use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug)]
struct Relation {
    sources: Range<usize>,
    destinations: Range<usize>,
}

#[derive(Debug)]
struct Map(Vec<Relation>);

fn trace_location(seed: usize, steps: &[Map]) -> usize {
    steps.iter().fold(seed, |acc, m| {
        m.0.iter()
            .find(|r| r.sources.contains(&acc))
            .map(|r| r.destinations.start + (acc - r.sources.start))
            .unwrap_or(acc)
    })
}

fn main() {
    let input = include_str!("./input.txt").lines().collect_vec();

    let steps = input[2..]
        .iter()
        .group_by(|line| !line.is_empty()) // Split on empty line between maps
        .into_iter()
        .map(|(_key, group)| group.collect_vec())
        .filter(|group| !group.iter().all(|s| s.is_empty())) // Skip empty lines
        .map(|group| {
            group
                .iter()
                .skip(1) // Skip the lines like "seed-to-soil map:"
                .map(|numbers| {
                    let (dest_start, src_start, count) = numbers.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect_tuple().unwrap();
                    Relation {
                        sources: src_start..src_start + count,
                        destinations: dest_start..dest_start + count,
                    }
                })
                .collect_vec()
        })
        .map(Map)
        .collect_vec();

    let seeds = input[0]
        .split_once("seeds: ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    let part1_res = seeds.par_iter().map(|seed| trace_location(*seed, &steps)).min().unwrap();

    println!("{part1_res}"); // 51580674

    let part2_seeds = seeds.chunks(2).map(|c| c[0]..c[0] + c[1]).collect_vec();
    let part2_res = part2_seeds
        .into_par_iter()
        .map(|range| range.map(|seed| trace_location(seed, &steps)).min().unwrap())
        .min()
        .unwrap();

    println!("{part2_res}");
}
